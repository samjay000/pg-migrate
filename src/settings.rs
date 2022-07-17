use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(unused)]
#[allow(unused_attributes)]
pub struct Postgresql {
    pub host: String,
    pub port: String,
    pub dbname: String,
    pub user: String,
    pub password: String,
    pub schema: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(unused)]
#[allow(unused_attributes)]
pub struct Files {
    pub file: Option<String>,
    pub files: Option<Vec<String>>,
    pub folder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(unused)]
#[allow(unused_attributes)]
pub struct Settings {
    pub postgresql: Postgresql,
    pub files: Files,
}

impl Settings {
    pub fn new_from_file(file: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .set_default("files.file", "schema.v1.sql")?
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(&file))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            // .add_source(
            //     File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode))
            //         .required(false),
            // )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            // .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("pg-sync"))
            .build()?;

        // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("postgresql.host"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .set_default("files.file", "schema.v1.sql")?
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("pg-sync"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            // .add_source(
            //     File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode))
            //         .required(false),
            // )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            // .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("pg-sync"))
            .build()?;

        // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("postgresql.host"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}