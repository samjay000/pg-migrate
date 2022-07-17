// #![allow(dead_code)]
#![allow(unused_variables)]

use itertools::Itertools;
use log::{debug, info};
use postgres::Client;
use sqlparser::ast;
use sqlparser::ast::{AlterColumnOperation, AlterTableOperation, ColumnDef, ColumnOptionDef, DataType, Expr, HiveDistributionStyle, Ident, ObjectName, ObjectType, Statement};
use sqlparser::ast::Expr::{Cast, Identifier};
use sqlparser::dialect::PostgreSqlDialect;

use crate::{file_loader, SQLParser};
use crate::plan::{Error, Plan};

pub fn apply_file(file: &str, schema: &String, client: &mut Client) -> Result<Plan, Error> {
    let mut plan = Plan::new();

    let contents = file_loader::load(file);

    let dialect = PostgreSqlDialect {}; // or AnsiDialect, or your own dialect ...
    let ast: Vec<ast::Statement> = SQLParser::parse_sql(&dialect, &contents).unwrap();
    info!("AST: {:?}", ast);

    for row_table in client.query("Select table_name from information_schema.tables where table_schema= $1 ", &[schema]).unwrap() {
        let table_name: &str = row_table.get(0);
        plan.table_names_all_from_db.push(table_name.to_string());
    }
    for statement in &ast {
        match statement {
            Statement::CreateTable { or_replace, temporary, external, global, if_not_exists, name, columns, constraints, hive_distribution, hive_formats, table_properties, with_options, file_format, location, query, without_rowid, like, engine, default_charset, collation, on_commit } => {
                plan.table_names_all_from_file.push(name.to_string());
            }
            _ => {}
        }
    }
    plan.table_names_dup_from_file = plan.table_names_all_from_file.clone().into_iter().duplicates().collect::<Vec<String>>();
    if !plan.table_names_dup_from_file.is_empty() { return Err(Error::DuplicateTableName { table_names: plan.table_names_dup_from_file.get(0).unwrap().to_string() }); }
    plan.table_names_unique_from_file = plan.table_names_all_from_file.clone().into_iter().unique().collect::<Vec<String>>();


    for row_table in client.query("Select table_name from information_schema.tables where table_schema= $1 ", &[schema]).unwrap() {
        let table_name: &str = row_table.get(0);
        for name in &plan.table_names_unique_from_file {
            if name == table_name {
                plan.table_names_existing.push(table_name.to_string());
            }
        }
        if !&plan.table_names_existing.contains(&table_name.to_string()) {
            plan.table_names_dropped.push(table_name.to_string())
        }
    }

    for name in &plan.table_names_unique_from_file {
        if !&plan.table_names_existing.contains(&name.to_string()) && !&plan.table_names_dropped.contains(&name.to_string()) {
            plan.table_names_new.push(name.to_string());
        }
    }

    for table_name in &plan.table_names_existing {
        let column_defs_from_db = make_column_def_by_table_name(&schema, client, &table_name);

        for statement in &ast {
            match statement {
                Statement::CreateTable { or_replace, temporary, external, global, if_not_exists, name, columns, constraints, hive_distribution, hive_formats, table_properties, with_options, file_format, location, query, without_rowid, like, engine, default_charset, collation, on_commit } => {
                    if name.to_string() == table_name.to_string() {
                        debug!("{:?}", columns);
                        let mut table_changes = diff_from_table_statements(name, columns.to_vec(), column_defs_from_db);
                        if table_changes.len() == 0 {
                            plan.table_names_unchanged.push(name.to_string());
                        }
                        plan.table_statements_changes.append(&mut table_changes);
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    for table_name in &plan.table_names_dropped {
        plan.table_statements_dropped.push(Statement::Drop {
            object_type: ObjectType::Table,
            if_exists: false,
            names: vec![ObjectName(vec![Ident { value: table_name.to_string(), quote_style: None }])],
            cascade: false,
            purge: false,
        })
    }

    for statement in &ast {
        match statement {
            Statement::CreateTable { or_replace, temporary, external, global, if_not_exists, name, columns, constraints, hive_distribution, hive_formats, table_properties, with_options, file_format, location, query, without_rowid, like, engine, default_charset, collation, on_commit } => {
                if plan.table_names_new.contains(&name.to_string()) {
                    &plan.table_statements_new.push(statement.clone());
                }
            }
            _ => {}
        }
    }
    make_sql_statements(&mut plan);
    make_reverse_plan(&mut plan, &schema, client);
    return Ok(plan);
}

fn make_column_def_by_table_name(schema: &&String, client: &mut Client, table_name: &String) -> Vec<ColumnDef> {
    let mut column_defs_from_db: Vec<ColumnDef> = vec![];
    for row_column in client.query("Select * from information_schema.columns where table_schema = $1 and table_name= $2 ", &[&schema, &table_name.to_string()]).unwrap() {
        let table_catalog: &str = row_column.get("table_catalog"); //Name of the database containing the table (always the current database)
        let table_schema: &str = row_column.get("table_schema"); //Name of the schema containing the table
        let table_name: &str = row_column.get("table_name"); //Name of the table
        let column_name: &str = row_column.get("column_name"); //Name of the column
        let ordinal_position: i32 = row_column.get("ordinal_position"); //Ordinal position of the column within the table (count starts at 1)
        let column_default: Option<&str> = row_column.get("column_default"); //Default expression of the column
        let is_nullable: bool = if row_column.get::<&str, &str>("is_nullable") == "YES" { true } else { false }; //YES if the column is possibly nullable, NO if it is known not nullable. A not-null constraint is one way a column can be known not nullable, but there can be others.
        let data_type: &str = row_column.get("data_type"); //Data type of the column, if it is a built-in type, or ARRAY if it is some array (in that case, see the view element_types), else USER-DEFINED (in that case, the type is identified in udt_name and associated columns). If the column is based on a domain, this column refers to the type underlying the domain (and the domain is identified in domain_name and associated columns).
        let character_maximum_length: Option<i32> = row_column.get("character_maximum_length"); //If data_type identifies a character or bit string type, the declared maximum length; null for all other data types or if no maximum length was declared.
        let character_octet_length: Option<i32> = row_column.get("character_octet_length"); //If data_type identifies a character type, the maximum possible length in octets (bytes) of a datum; null for all other data types. The maximum octet length depends on the declared character maximum length (see above) and the server encoding.
        let numeric_precision: Option<i32> = row_column.get("numeric_precision"); //If data_type identifies a numeric type, this column contains the (declared or implicit) precision of the type for this column. The precision indicates the number of significant digits. It can be expressed in decimal (base 10) or binary (base 2) terms, as specified in the column numeric_precision_radix. For all other data types, this column is null.
        let numeric_precision_radix: Option<i32> = row_column.get("numeric_precision_radix"); //If data_type identifies a numeric type, this column indicates in which base the values in the columns numeric_precision and numeric_scale are expressed. The value is either 2 or 10. For all other data types, this column is null.
        let numeric_scale: Option<i32> = row_column.get("numeric_scale"); //If data_type identifies an exact numeric type, this column contains the (declared or implicit) scale of the type for this column. The scale indicates the number of significant digits to the right of the decimal point. It can be expressed in decimal (base 10) or binary (base 2) terms, as specified in the column numeric_precision_radix. For all other data types, this column is null.
        let datetime_precision: Option<i32> = row_column.get("datetime_precision"); //If data_type identifies a date, time, timestamp, or interval type, this column contains the (declared or implicit) fractional seconds precision of the type for this column, that is, the number of decimal digits maintained following the decimal point in the seconds value. For all other data types, this column is null.
        let interval_type: Option<&str> = row_column.get("interval_type"); //If data_type identifies an interval type, this column contains the specification which fields the intervals include for this column, e.g., YEAR TO MONTH, DAY TO SECOND, etc. If no field restrictions were specified (that is, the interval accepts all fields), and for all other data types, this field is null.
        let interval_precision: Option<i32> = row_column.get("interval_precision"); //Applies to a feature not available in PostgreSQL (see datetime_precision for the fractional seconds precision of interval type columns)
        let character_set_catalog: Option<&str> = row_column.get("character_set_catalog"); //Applies to a feature not available in PostgreSQL
        let character_set_schema: Option<&str> = row_column.get("character_set_schema"); //Applies to a feature not available in PostgreSQL
        let character_set_name: Option<&str> = row_column.get("character_set_name"); //Applies to a feature not available in PostgreSQL
        let collation_catalog: Option<&str> = row_column.get("collation_catalog"); //Name of the database containing the collation of the column (always the current database), null if default or the data type of the column is not collatable
        let collation_schema: Option<&str> = row_column.get("collation_schema"); //Name of the schema containing the collation of the column, null if default or the data type of the column is not collatable
        let collation_name: Option<&str> = row_column.get("collation_name"); //Name of the collation of the column, null if default or the data type of the column is not collatable
        let domain_catalog: Option<&str> = row_column.get("domain_catalog"); //If the column has a domain type, the name of the database that the domain is defined in (always the current database), else null.
        let domain_schema: Option<&str> = row_column.get("domain_schema"); //If the column has a domain type, the name of the schema that the domain is defined in, else null.
        let domain_name: Option<&str> = row_column.get("domain_name"); //If the column has a domain type, the name of the domain, else null.
        let udt_catalog: Option<&str> = row_column.get("udt_catalog"); //Name of the database that the column data type (the underlying type of the domain, if applicable) is defined in (always the current database)
        let udt_schema: Option<&str> = row_column.get("udt_schema"); //Name of the schema that the column data type (the underlying type of the domain, if applicable) is defined in
        let udt_name: Option<&str> = row_column.get("udt_name"); //Name of the column data type (the underlying type of the domain, if applicable)
        let maximum_cardinality: Option<i32> = row_column.get("maximum_cardinality"); //Always null, because arrays always have unlimited maximum cardinality in PostgreSQL
        let dtd_identifier: Option<&str> = row_column.get("dtd_identifier"); //An identifier of the data type descriptor of the column, unique among the data type descriptors pertaining to the table. This is mainly useful for joining with other instances of such identifiers. (The specific format of the identifier is not defined and not guaranteed to remain the same in future versions.)
        let is_identity: Option<&str> = row_column.get("is_identity"); //If the column is an identity column, then YES, else NO.
        let identity_generation: Option<&str> = row_column.get("identity_generation"); //If the column is an identity column, then ALWAYS or BY DEFAULT, reflecting the definition of the column.
        let identity_start: Option<&str> = row_column.get("identity_start"); //If the column is an identity column, then the start value of the internal sequence, else null.
        let identity_increment: Option<&str> = row_column.get("identity_increment"); //If the column is an identity column, then the increment of the internal sequence, else null.
        let identity_maximum: Option<&str> = row_column.get("identity_maximum"); //If the column is an identity column, then the maximum value of the internal sequence, else null.
        let identity_minimum: Option<&str> = row_column.get("identity_minimum"); //If the column is an identity column, then the minimum value of the internal sequence, else null.
        let identity_cycle: Option<&str> = row_column.get("identity_cycle"); //If the column is an identity column, then YES if the internal sequence cycles or NO if it does not; otherwise null.
        let is_generated: Option<&str> = row_column.get("is_generated"); //If the column is a generated column, then ALWAYS, else NEVER.
        let generation_expression: Option<&str> = row_column.get("generation_expression"); //If the column is a generated column, then the generation expression, else null.
        let is_updatable: Option<&str> = row_column.get("is_updatable"); //YES if the column is updatable, NO if not (Columns in base tables are always updatable, columns in views not necessarily)


        let dt = match data_type {
            "integer" => { DataType::Int(None) }
            "text" => { DataType::Text }
            &_ => { DataType::Text }
        };
        debug!("{}, {}", column_name, data_type);
        let stmt_column = ColumnDef { name: Ident { value: column_name.to_string(), quote_style: None }, data_type: dt, collation: None, options: vec![] };
        column_defs_from_db.push(stmt_column);
    }
    column_defs_from_db
}

fn diff_from_table_statements(table_name: &ObjectName, from_file: Vec<ColumnDef>, from_db: Vec<ColumnDef>) -> Vec<Statement> {
    let mut table_statement_updated: Vec<Statement> = vec![];
    for column_file in &from_file {
        for column_db in &from_db {
            debug!("{},{}", column_file, column_db);
            if column_file.name.value == column_db.name.value {
                debug!("diff_from_table_statements: {},{}", column_file, column_db);
                if column_file.data_type != column_db.data_type {
                    table_statement_updated.push(
                        Statement::AlterTable {
                            name: table_name.clone(),
                            operation: AlterTableOperation::DropColumn {
                                column_name: column_file.name.clone(),
                                if_exists: false,
                                cascade: false,
                            },
                        }
                    );
                    table_statement_updated.push(
                        Statement::AlterTable {
                            name: table_name.clone(),
                            operation: AlterTableOperation::AddColumn {
                                column_def: ColumnDef {
                                    name: column_file.name.clone(),
                                    data_type: column_file.data_type.clone(),
                                    collation: None,
                                    options: vec![],
                                },
                            },
                        }
                    );
                    // With cast
                    // table_statement_updated.push(
                    //     Statement::AlterTable {
                    //         name: table_name.clone(),
                    //         operation: AlterTableOperation::AlterColumn {
                    //             column_name: column_file.name.clone(),
                    //             op: AlterColumnOperation::SetDataType { data_type: column_file.data_type.clone(), using:
                    //             // Box::<>(Identifier(Ident { value: column_file.name.value, quote_style: None })
                    //             Some(Cast { expr: Box::new(Expr::Identifier(column_file.name.clone())), data_type: column_file.data_type.clone() })
                    //             },
                    //         },
                    //     }
                    // );
                }
                break;
            }
        }
    }
    return table_statement_updated;
}


fn make_sql_statements(plan: &mut Plan) {
    for table_statement_dropped in &plan.table_statements_dropped {
        plan.sql_statements_for_step_up.push(table_statement_dropped.to_string());
    }

    for table_statements_change in &plan.table_statements_changes {
        plan.sql_statements_for_step_up.push(table_statements_change.to_string());
    }

    for table_statement_new in &plan.table_statements_new {
        plan.sql_statements_for_step_up.push(table_statement_new.to_string());
    }
}

fn make_reverse_plan(plan: &mut Plan, schema: &&String, client: &mut Client) {
    for table_statement_dropped in &plan.table_statements_dropped {
        match table_statement_dropped {
            Statement::Drop { object_type, if_exists, names, cascade, purge } => {
                let table_name = names[0].to_string();
                debug!("{}",table_name);
                // let column_defs = make_column_def_by_table_name(schema,     &mut client, &&table_name);
                // let statement = Statement::CreateTable {
                //     or_replace: false,
                //     temporary: false,
                //     external: false,
                //     global: None,
                //     if_not_exists: false,
                //     name: ObjectName(vec![Ident { value: table_name, quote_style: None }]),
                //     columns: column_defs,
                //     constraints: vec![],
                //     hive_distribution: HiveDistributionStyle::NONE,
                //     hive_formats: None,
                //     table_properties: vec![],
                //     with_options: vec![],
                //     file_format: None,
                //     location: None,
                //     query: None,
                //     without_rowid: false,
                //     like: None,
                //     engine: None,
                //     default_charset: None,
                //     collation: None,
                //     on_commit: None,
                // };
                // plan.sql_statements_for_step_down.push(statement.to_string());
            }
            _ => {}
        }
    }

    for table_statements_change in &plan.table_statements_changes {
        match table_statements_change {
            Statement::Drop { object_type, if_exists, names, cascade, purge } => {}
            _ => {}
        }
    }

    for table_statement_new in &plan.table_statements_new {
        match table_statement_new {
            Statement::CreateTable { or_replace, temporary, external, global, if_not_exists, name, columns, constraints, hive_distribution, hive_formats, table_properties, with_options, file_format, location, query, without_rowid, like, engine, default_charset, collation, on_commit } => {
                plan.sql_statements_for_step_down.push(Statement::Drop {
                    object_type: ObjectType::Table,
                    if_exists: false,
                    names: vec![name.to_owned()],
                    cascade: false,
                    purge: false,
                }.to_string())
            }
            _ => {}
        }
    }
}