#[cfg(test)]
use test_env_helpers::*;

#[cfg(test)]
#[before_all]
mod tests {
    use log::{debug, info};
    use sqlparser::ast::{ColumnDef, HiveDistributionStyle, HiveFormat, Ident, ObjectName};
    use sqlparser::ast::DataType::Text;
    use sqlparser::ast::Statement::CreateTable;

    use pg_sync;

    fn before_all() {
        pg_sync::setup_logger(log::LevelFilter::Debug).expect("Setting up logger failed with panic!.");
    }

    #[test]
    fn test_1_new_table() {
        debug!("test_1_new_table");
        let mut settings = pg_sync::make_settings(&"pg-sync".to_string());

        settings.files.file = Some("schema.v1.sql".to_string());
        let result = pg_sync::apply_file(&settings, pg_sync::db_connection::make_connection(&settings.postgresql));
        info!("{:?}",result);
        let mut plan = pg_sync::plan::Plan::new();
        plan.table_names_all_from_file.push("table1".to_string());
        plan.table_names_unique_from_file.push("table1".to_string());
        plan.table_names_new.push("table1".to_string());
        plan.table_statements_new.push(
            CreateTable { or_replace: false, temporary: false, external: false, global: None, if_not_exists: false, name: ObjectName(vec![Ident { value: "table1".to_string(), quote_style: None }]), columns: vec![ColumnDef { name: Ident { value: "column11".to_string(), quote_style: None }, data_type: Text, collation: None, options: vec![] }], constraints: vec![], hive_distribution: HiveDistributionStyle::NONE, hive_formats: Some(HiveFormat { row_format: None, storage: None, location: None }), table_properties: vec![], with_options: vec![], file_format: None, location: None, query: None, without_rowid: false, like: None, engine: None, default_charset: None, collation: None, on_commit: None }
        );
        plan.sql_statements_for_step_up.push("CREATE TABLE table1 (column11 TEXT)".to_string());
        plan.sql_statements_for_step_down.push("DROP TABLE table1".to_string());
        let mut client = pg_sync::db_connection::make_connection(&settings.postgresql);
        plan.apply_plan_up(&mut client);
        plan.apply_plan_down(&mut client);
        assert_eq!(format!("{:?}", plan), format!("{:?}", result.unwrap()))
    }

    // #[test]
    // fn test_2_new_table() {
    //     pg_sync::setup_logger().expect("Setting up logger failed with panic!.");
    //     debug!("test_2_new_table");
    //     let mut settings = pg_sync::make_settings("pg-sync".to_string());
    //     settings.files.file = Some("schema.v1.sql".to_string());
    //     let  result = pg_sync::apply_file(&settings);
    //     debug!("{:?}",result);
    //     let mut plan = pg_sync::plan::Plan::new();
    //     plan.table_names_all_from_file.push("table1".to_string());
    //     plan.table_names_unique_from_file.push("table1".to_string());
    //     plan.table_names_new.push("table1".to_string());
    //     plan.table_statements_new.push(
    //         CreateTable { or_replace: false, temporary: false, external: false, global: None, if_not_exists: false, name: ObjectName(vec![Ident { value: "table1".to_string(), quote_style: None }]), columns: vec![ColumnDef { name: Ident { value: "column11".to_string(), quote_style: None }, data_type: Text, collation: None, options: vec![] }], constraints: vec![], hive_distribution: HiveDistributionStyle::NONE, hive_formats: Some(HiveFormat { row_format: None, storage: None, location: None }), table_properties: vec![], with_options: vec![], file_format: None, location: None, query: None, without_rowid: false, like: None, engine: None, default_charset: None, collation: None, on_commit: None }
    //     );
    //     assert_eq!(format!("{:?}",plan) , format!("{:?}",result.unwrap()))
    // }
}