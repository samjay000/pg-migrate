use postgres::{Client, NoTls};
use shadow_rs::shadow;

mod tests;

shadow!(build);

pub fn connect(
    user: String,
    password: String,
    host: String,
    port: String,
    database: String,
) -> Client {
    Client::connect(
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        )
        .as_str(),
        NoTls,
    )
    .unwrap()
}

pub fn is_function_exists(mut client: Client, function_name: &str) -> bool {
    let mut result = false;
    for row in client.query("SELECT count(routine_name) FROM information_schema.routines WHERE routine_type = 'FUNCTION' AND routine_name=$1", &[&function_name]).unwrap() {
        let val: i64 = row.get(0);
        if val > 0 {
            result = true;
        }
    }
    return result;
}

pub fn create_function_if_not_exist(mut client: Client, function_name: &str) -> bool {
    let result = is_function_exists(client, function_name);

    return result;
}
