use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "sql"]
struct Asset;

pub fn get_version_sql(version: &str) -> String {
    let embedded_file = Asset::get("get_version.sql").unwrap();
    let local_variable = String::from(std::str::from_utf8(embedded_file.data.as_ref()).unwrap());
    return local_variable.replace("{version}", version);
}

pub fn get_table_definition_sql() -> String {
    let embedded_file = Asset::get("get_table_definition.sql").unwrap();
    let local_variable = String::from(std::str::from_utf8(embedded_file.data.as_ref()).unwrap());
    return local_variable;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_sql() {
        let version = "v0.0.1";
        let result = get_version_sql(version);
        assert_eq!(result, "CREATE OR REPLACE FUNCTION public.get_pg_migrate_version(
)
    RETURNS SETOF TEXT
AS
$BODY$
BEGIN
    RETURN query
        SELECT '{version}';
END;
$BODY$
    LANGUAGE plpgsql;


select get_pg_migrate_version();".replace("{version}", version));
    }
}
