use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "sql"]
struct Asset;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let get_version_sql = Asset::get("get_version.sql").unwrap();
        let local_variable = std::str::from_utf8(get_version_sql.data.as_ref()).unwrap();
        println!("get_version_sql: {local_variable}");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
