CREATE OR REPLACE FUNCTION public.get_pg_migrate_version(
)
    RETURNS SETOF TEXT
AS
$BODY$
BEGIN
    RETURN query
        SELECT 'v0.1.0';
END;
$BODY$
    LANGUAGE plpgsql;


select get_pg_migrate_version();