CREATE OR REPLACE FUNCTION public.get_pg_migrate_version(
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


select get_pg_migrate_version();