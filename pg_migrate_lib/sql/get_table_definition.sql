CREATE OR REPLACE FUNCTION public.get_table_def(
    p_schema_name character varying,
    p_table_name character varying
)
    RETURNS SETOF TEXT
AS
$BODY$
BEGIN
    RETURN query
        WITH table_rec AS (SELECT c.relname,
                                  n.nspname,
                                  c.oid
                           FROM pg_catalog.pg_class c
                                    LEFT JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                           WHERE relkind = 'r'
                             AND n.nspname = p_schema_name
                             AND c.relname LIKE p_table_name
                           ORDER BY c.relname),
             col_rec AS (SELECT a.attname                                       AS colname,
                                pg_catalog.format_type(a.atttypid, a.atttypmod) AS coltype,
                                a.attrelid                                      AS oid,
                                ' DEFAULT ' || (SELECT pg_catalog.pg_get_expr(d.adbin, d.adrelid)
                                                FROM pg_catalog.pg_attrdef d
                                                WHERE d.adrelid = a.attrelid
                                                  AND d.adnum = a.attnum
                                                  AND a.atthasdef)              AS column_default_value,
                                CASE
                                    WHEN a.attnotnull = TRUE THEN
                                        'NOT NULL'
                                    ELSE
                                        'NULL'
                                    END                                         AS column_not_null,
                                a.attnum                                        AS attnum
                         FROM pg_catalog.pg_attribute a
                         WHERE a.attnum > 0
                           AND NOT a.attisdropped
                         ORDER BY a.attnum),
             con_rec AS (SELECT conrelid::regclass::text    AS relname,
                                n.nspname,
                                conname,
                                pg_get_constraintdef(c.oid) AS condef,
                                contype,
                                conrelid                    AS oid
                         FROM pg_constraint c
                                  JOIN pg_namespace n ON n.oid = c.connamespace),
             glue AS (SELECT format(
                                     E'-- %1$I.%2$I definition\n\n-- Drop table\n\n-- DROP TABLE IF EXISTS %1$I.%2$I\n\nCREATE TABLE %1$I.%2$I (\n',
                                     table_rec.nspname, table_rec.relname) AS top,
                             format(E'\n);\n\n\n-- adempiere.wmv_ghgaudit foreign keys\n\n', table_rec.nspname,
                                    table_rec.relname)                     AS bottom,
                             oid
                      FROM table_rec),
             cols AS (SELECT string_agg(
                                     format('    %I %s%s %s', colname, coltype, column_default_value, column_not_null),
                                     E',\n') AS lines,
                             oid
                      FROM col_rec
                      GROUP BY oid),
             constrnt AS (SELECT string_agg(format('    CONSTRAINT %s %s', con_rec.conname, con_rec.condef),
                                            E',\n') AS lines,
                                 oid
                          FROM con_rec
                          WHERE contype <> 'f'
                          GROUP BY oid),
             frnkey AS (SELECT string_agg(format('ALTER TABLE %I.%I ADD CONSTRAINT %s %s', nspname, relname, conname,
                                                 condef), E';\n') AS lines,
                               oid
                        FROM con_rec
                        WHERE contype = 'f'
                        GROUP BY oid)
        SELECT concat(glue.top, cols.lines, E',\n', constrnt.lines, glue.bottom, frnkey.lines, ';')
        FROM glue
                 JOIN cols ON cols.oid = glue.oid
                 LEFT JOIN constrnt ON constrnt.oid = glue.oid
                 LEFT JOIN frnkey ON frnkey.oid = glue.oid;
END;
$BODY$
    LANGUAGE plpgsql;


select get_tabledef(' public', 'table1');