## PG-Sync

PG-Sync is a PostgreSQL schema chnage management tool.

#### Why?

Other existing DB schema management tools deals with change sets or change set files, you dont have currunt structure of
your schema in sigle place(file). Either you go through change sets or use database client tool(PG Admin).

#### How?

With PG-Sync you will have current database schema structure in file and PG_Sync will read your database structure and
your schema file and apply diff on your database.
*current database schema structure = Schema if you do a fresh installation of current version of you application.*

#### Installation

`cargo install pg_sync
`

#### Example 1

```sql
CREATE TABLE table1
(
    column11 text
);
```

Will produce

```sql
CREATE TABLE table1
(
    column11 text
);
```

#### Example 2

```sql
CREATE TABLE table1
(
    column11 text,
    column12 text
);
```

Will produce

```sql
ALTER TABLE table1
    ADD COLUMN column12 text;
```