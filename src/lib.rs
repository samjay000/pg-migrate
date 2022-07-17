use std::str::FromStr;

use clap::Parser;
use config::ConfigError;
use log::debug;
use postgres::Client;
use sqlparser::ast::Statement;
use sqlparser::parser::Parser as SQLParser;
use version::{Version, version};

use crate::arguments::Args;
use crate::plan::{Error, Plan};
use crate::settings::Settings;

mod file_loader;
pub mod settings;
mod arguments;
pub mod db_connection;
mod apply_file;
pub mod plan;

pub fn start_processing() {
    let args: Args = arguments::Args::parse();
    setup_logger(&args).expect("Setting up logger failed with panic!.");
    debug!("{:?}",args);
    print_heading();

    let settings = make_settings(&args.config);
    apply_file_and_print_summary(&args, &settings);
}

pub fn make_settings(file_name: &String) -> Settings {
    let settings = settings::Settings::new_from_file(file_name.to_string());
    match settings {
        Ok(settings) => {
            return settings;
        }
        Err(error) => {
            debug!("{:?}",error);
            bunt::println!("{$bold+red}Error:   {:?}{/$}", error);
            std::process::exit(0)
        }
    }
}

pub fn apply_file_and_print_summary(args: &Args, settings: &settings::Settings) {
    print_postgresql_connection_info(&settings);
    let mut client = db_connection::make_connection(&settings.postgresql);
    let result = apply_file(settings, client);
    match result {
        Ok(plan) => {
            bunt::println!();
            print_plan_details(&plan);
            bunt::println!();
            print_plan_summary(&plan);
            if args.apply {
                yes_apply_changes(&plan, &mut db_connection::make_connection(&settings.postgresql));
            } else {
                ask_do_you_want_to_apply_up(&plan, &mut db_connection::make_connection(&settings.postgresql));
            }
        }
        Err(error) => {
            bunt::println!("{$bold+red}{} {/$}.", error);
        }
    }
}

pub fn apply_file(settings: &settings::Settings, mut client: Client) -> Result<Plan, Error> {
    let result = apply_file::apply_file(&settings.files.file.as_ref().unwrap(), &settings.postgresql.schema.as_ref().unwrap_or(&"public".to_string()).to_string(), &mut client);
    return result;
}

pub fn print_plan_details(plan: &Plan) {
    bunt::println!("{$bold}Plan details: {/$}");
    bunt::println!("{} new table(s)", plan.table_statements_new.len());
    for table_statement_new in &plan.table_statements_new {
        bunt::println!("    {:?}", table_statement_new.to_string());
    }
    bunt::println!("{} table(s) will drop", plan.table_statements_dropped.len());
    for table_statement_dropped in &plan.table_statements_dropped {
        bunt::println!("    {:?}", table_statement_dropped.to_string());
    }
    bunt::println!("{} table(s) unchanged", plan.table_names_unchanged.len());
    bunt::println!("    {:?}", plan.table_names_unchanged);
    bunt::println!("{} table(s) changed", plan.table_statements_changes.len());
    for table_statement_change in &plan.table_statements_changes {
        bunt::println!("    {:?}", table_statement_change.to_string());
    }
}

pub fn print_plan_summary(plan: &Plan) {
    bunt::println!("{$bold}Plan summary: {/$}");
    bunt::println!("{} new table(s)", plan.table_statements_new.len());
    bunt::println!("    {:?}", plan.table_names_new);
    bunt::println!("{} table(s) will drop", plan.table_names_dropped.len());
    bunt::println!("    {:?}", plan.table_names_dropped);
    bunt::println!("{} table(s) unchanged", plan.table_names_unchanged.len());
    bunt::println!("    {:?}", plan.table_names_unchanged);
    bunt::println!("{} table(s) changed", plan.table_statements_changes.len());
    let mut table_names_changed: Vec<String> = vec![];
    for table_statement_change in &plan.table_statements_changes {
        match table_statement_change {
            Statement::AlterTable { name, operation } => {
                if !table_names_changed.contains(&name.to_string()) { table_names_changed.push(name.to_string()); }
            }
            _ => {}
        }
    }
    bunt::println!("    {:?}", table_names_changed);
}

pub fn print_postgresql_connection_info(settings: &Settings) {
    bunt::println!("{$italic}Conneting to {}@{}:{}/{} {/$}", settings.postgresql.user, settings.postgresql.host, settings.postgresql.port, settings.postgresql.dbname);
}

pub fn print_heading() {
    let ver: Version = FromStr::from_str(version!()).unwrap();
    bunt::println!("{$bold}PG Sync{/$} - Version {}", ver);
}

pub fn setup_logger(args: &Args) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                // chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(args.log_level)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn ask_do_you_want_to_apply_up(plan: &Plan, client: &mut Client) {
    bunt::println!("{$bold}Do want to apply above changes({/$}yes or no{$bold})? {/$}");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(n) => {
            match input.as_str().trim() {
                "yes" => {
                    yes_apply_changes(plan, client);
                }
                _ => {
                    no_do_not_apply_changes();
                }
            }
        }
        Err(error) => println!("error: {error}"),
    }
}

fn no_do_not_apply_changes() {
    bunt::println!("Not applying changes.");
    bunt::println!("Done.");
    std::process::exit(0);
}

fn yes_apply_changes(plan: &Plan, client: &mut Client) {
    bunt::println!("Applying changes.");
    plan.apply_plan_up(client);
    bunt::println!("Done.");
}

