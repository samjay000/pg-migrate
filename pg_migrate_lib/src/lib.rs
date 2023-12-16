use postgres::{Client, NoTls};

pub fn connect(user: String, password: String, host: String, port: String, database: String) -> Client {
    Client::connect(
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            user,
            password,
            host,
            port,
            database
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        connect("samjay".to_string(), "".to_string(), "localhost".to_string(), "5432".to_string(), "postgres".to_string());
    }

    #[test]
    fn test_is_function_exists1() {
        let con = connect("samjay".to_string(), "".to_string(), "localhost".to_string(), "5432".to_string(), "postgres".to_string());
        let res = is_function_exists(con, "get_pg_migrate_version");
        println!("is_function_exists1: {}", res)
    }

    #[test]
    fn test_is_function_exists2() {
        let con = connect("samjay".to_string(), "".to_string(), "localhost".to_string(), "5432".to_string(), "postgres".to_string());
        let res = is_function_exists(con, "get_table_def");
        println!("is_function_exists2: {}", res)
    }
}
