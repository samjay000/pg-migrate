CREATE TABLE table1
(
    column11 integer PRIMARY KEY,
    column12 text
);

CREATE TABLE table2
(
    column21 bigint,
    column22 bigint,
    PRIMARY KEY (column21, column22)
);
