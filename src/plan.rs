use postgres::Client;
use snafu::prelude::*;
use sqlparser::ast::Statement;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Duplicate table definition(s) found, {table_names}"))]
    DuplicateTableName { table_names: String },
}

#[derive(Debug)]
pub struct Plan {
    pub table_names_all_from_file: Vec<String>,
    pub table_names_all_from_db: Vec<String>,
    pub table_names_unique_from_file: Vec<String>,
    pub table_names_dup_from_file: Vec<String>,
    pub table_names_existing: Vec<String>,
    pub table_names_unchanged: Vec<String>,
    pub table_statements_changes: Vec<Statement>,
    pub table_names_new: Vec<String>,
    pub table_statements_new: Vec<Statement>,
    pub table_names_dropped: Vec<String>,
    pub table_statements_dropped: Vec<Statement>,
    pub tables_new: Vec<Statement>,
    pub tables_old: Vec<Statement>,
    pub sql_statements_for_step_up: Vec<String>,
    pub sql_statements_for_step_down: Vec<String>,
}

impl Plan {
    pub fn new() -> Plan {
        return Plan {
            table_names_all_from_file: vec![],
            table_names_all_from_db: vec![],
            table_names_unique_from_file: vec![],
            table_names_dup_from_file: vec![],
            table_names_existing: vec![],
            table_names_unchanged: vec![],
            table_statements_changes: vec![],
            table_names_new: vec![],
            table_statements_new: vec![],
            table_names_dropped: vec![],
            table_statements_dropped: vec![],
            tables_new: vec![],
            tables_old: vec![],
            sql_statements_for_step_up: vec![],
            sql_statements_for_step_down: vec![]
        };
    }

    pub fn apply_plan_up(self: &Plan, client: &mut Client) {
        for step in &self.sql_statements_for_step_up {
            let _ = client.execute(step.as_str(), &[]);
        }
    }
    pub fn apply_plan_down(self: &Plan, client: &mut Client) {
        for step in &self.sql_statements_for_step_down {
            let _ = client.execute(step.as_str(), &[]);
        }
    }
}
