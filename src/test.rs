use pest::prelude::*;
use super::Rdp;

macro_rules! assert_parse {
    ($s:expr, $c:ident) => {{
        let mut parser = Rdp::new(StringInput::new($s));
        assert!(parser.$c());
        assert!(parser.end());
    }};
}

#[test]
fn test_alter_table() {
    assert_parse!("ALTER TABLE test RENAME TO new", alter_table);
    assert_parse!("ALTER TABLE main.test RENAME TO new", alter_table);

    assert_parse!("ALTER TABLE test ADD new", alter_table);
    assert_parse!("ALTER TABLE test ADD COLUMN new", alter_table);

    //assert_parse!("ALTER TABLE RENAME TO new", alter_table);
}

#[test]
fn test_create_table() {
    assert_parse!("CREATE TABLE test (col)", create_table);
    assert_parse!("CREATE TABLE main.test (col)", create_table);
    assert_parse!("CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL)", create_table);

    //assert_parse!("CREATE TABLE test (id INTERGER NOT NULL, PRIMARY KEY (id))", create_table);
    assert_parse!("CREATE TABLE test AS SELECT 1", create_table);

    assert_parse!("CREATE TEMP TABLE test (col)", create_table);
    assert_parse!("CREATE TABLE IF NOT EXISTS test (col)", create_table);

    //assert_parse!(("CREATE TABLE test", create_table);
    //assert_parse!(("CREATE TABLE test ()", create_table);
    //assert_parse!(("CREATE TABLE test (PRIMARY KEY (id))", create_table);
    //assert_parse!(("CREATE TABLE test (col,)", create_table);
}

#[test]
fn test_column_definition() {
    assert_parse!("CREATE TABLE test (id UNSIGNED BIG INT)", create_table);
    assert_parse!("CREATE TABLE test (id INT8)", create_table);
    assert_parse!("CREATE TABLE test (id CHARACTER(20))", create_table);
    assert_parse!("CREATE TABLE test (id VARYING CHARACTER(255))", create_table);
    assert_parse!("CREATE TABLE test (id DOUBLE PRECISION)", create_table);
    assert_parse!("CREATE TABLE test (id DECIMAL(10,5))", create_table);
}

#[test]
fn test_column_constraints() {
    assert_parse!("CREATE TABLE test (id CONSTRAINTS not_null NOT NULL)", create_table);
    assert_parse!("CREATE TABLE test (id INTEGER PRIMARY KEY AUTOINCREMENT)", create_table);
    assert_parse!("CREATE TABLE test (id INTEGER PRIMARY KEY ON CONFLICT IGNORE)", create_table);
    assert_parse!("CREATE TABLE test (id UNIQUE)", create_table);
    //assert_parse!("CREATE TABLE test (id CHECK (id > 0))", create_table);
    assert_parse!("CREATE TABLE test (id DEFAULT '')", create_table);
    assert_parse!("CREATE TABLE test (id COLLATE NOCASE)", create_table);
    assert_parse!("REFERENCES fktable(id)", column_constraint);
    //assert_parse!("CREATE TABLE test (id REFERENCES fktable(id))", create_table);
    //assert_parse!("CREATE TABLE test (id REFERENCES fktable(id) ON DELETE CASCADE)", create_table);
}

#[test]
fn test_table_constraints() {
    //assert_parse!("CREATE TABLE test (id, CONSTRAINTS pk PRIMARY KEY (id))", create_table);
    //assert_parse!("CREATE TABLE test (id, UNIQUE (id))", create_table);
    //assert_parse!("CREATE TABLE test (id, CHECK (id > 0))", create_table);
    //assert_parse!("CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable(id))", create_table);
    //assert_parse!("CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable)", create_table);
    //assert_parse!("CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable(id) DEFERRABLE INITIALLY DEFERRED)", create_table);
}

#[test]
fn test_select() {
    assert_parse!("SELECT 1", cmd);
}

#[test]
fn test_one_select() {
    assert_parse!("SELECT 1", one_select);
    assert_parse!("SELECT 1, 'test'", one_select);

    assert_parse!("WHERE 1", where_clause);
    //assert_parse!("SELECT * FROM test WHERE 1", one_select);
    //assert_parse!("SELECT * FROM test WHERE 1 GROUP BY id HAVING count(*) > 1", one_select);
    //assert_parse!("SELECT * FROM test ORDER BY 1", one_select);
    //assert_parse!("SELECT * FROM test ORDER BY 1, id", one_select);
    //assert_parse!("SELECT * FROM test LIMIT 1", one_select);

    //assert_parse!("SELECT 1 FROM WHERE 1", one_select);
}

#[test]
fn test_select_column() {
    assert_parse!("1", select_column);
}

#[test]
fn test_expr() {
    assert_parse!("1", expr);
}

#[test]
fn test_number() {
    assert_parse!("1", number);
}
