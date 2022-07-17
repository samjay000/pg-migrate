use postgres::{Client, NoTls};

use crate::settings::Postgresql;

pub fn make_connection(config: &Postgresql) -> Client {
    let client = Client::connect("host=localhost user=samjay dbname=postgres ", NoTls).unwrap();
    return client;
}