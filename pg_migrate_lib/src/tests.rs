use crate::{connect, is_function_exists};

#[test]
fn test_connect() {
    connect(
        "samjay".to_string(),
        "".to_string(),
        "localhost".to_string(),
        "5432".to_string(),
        "postgres".to_string(),
    );
}

#[test]
fn test_is_function_exists1() {
    let con = connect(
        "samjay".to_string(),
        "".to_string(),
        "localhost".to_string(),
        "5432".to_string(),
        "postgres".to_string(),
    );
    let res = is_function_exists(con, "get_pg_migrate_version");
    println!("is_function_exists1: {}", res)
}

#[test]
fn test_is_function_exists2() {
    let con = connect(
        "samjay".to_string(),
        "".to_string(),
        "localhost".to_string(),
        "5432".to_string(),
        "postgres".to_string(),
    );
    let res = is_function_exists(con, "get_table_def");
    println!("is_function_exists2: {}", res)
}
