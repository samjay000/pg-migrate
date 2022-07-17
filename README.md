# pg-sync

PostgreSQL: `jdbc:postgresql://localhost:5432/postgres`  
1 table(s).

n of undoable migrations

**migrations**  
_create tables_  
create table table1  
_alter add columns_  
_alter change columns_  
_alter drop columns_

Undo migrations  
drop table table1  
_alter drop columns_  
_alter change columns_  
_alter add columns_

migration id n+1 run `pg-sync undo n+1` to undo changes

