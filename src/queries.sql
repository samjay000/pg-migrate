SELECT t.table_name
FROM information_schema.tables t
         JOIN information_schema.columns c ON t.table_name = c.table_name
WHERE t.table_schema = 'public';

SELECT t.table_name
FROM information_schema.tables t
         JOIN information_schema.columns c ON t.table_name = c.table_name
WHERE t.table_schema = 'public';

Select *
from information_schema.tables
where table_schema = 'public';
Select table_name
from information_schema.tables
where table_schema = 'public';
Select column_name, data_type
from information_schema.columns
where table_name = 'table1';
