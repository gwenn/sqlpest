use super::{Rule, SqlParser};
use pest::Parser;

// TODO https://docs.rs/pest/1.0.6/pest/macro.parses_to.html
macro_rules! assert_parse {
    ($s:expr, $c:ident) => {{
        let pairs = SqlParser::parse(Rule::$c, $s).unwrap_or_else(|e| panic!("{}", e));
        //println!("{:?} =>", $s);
        for pair in pairs.flatten() {
            let _span = pair.clone().into_span();
            // A pair is a combination of the rule which matched and a span of input
            //println!("Rule:    {:?}", pair.as_rule());
            //println!("Span:    {:?}", span);
            //println!("Text:    {}", span.as_str());
        }
    }};
}

#[test]
fn test_begin() {
    parses_to! {
        parser: SqlParser,
        input:  "BEGIN",
        rule:   Rule::begin,
        tokens: [
            begin(0, 5)
        ]
    };
    /*parses_to! {
        parser: SqlParser,
        input:  "BEGIN test",
        rule:   Rule::begin,
        tokens: [
            begin(0, 6, [id(6, 10)])
        ]
    };*/
}

#[test]
fn test_alter_table() {
    assert_parse!("ALTER TABLE test RENAME TO new", alter_table);
    assert_parse!("ALTER TABLE main.test RENAME TO new", alter_table);

    assert_parse!("ALTER TABLE test ADD new", alter_table);
    assert_parse!("ALTER TABLE test ADD COLUMN new", alter_table);

    fails_with! {
        parser: SqlParser,
        input: "ALTER TABLE RENAME TO new",
        rule: Rule::alter_table,
        positives: vec![Rule::alter_table_body],
        negatives: vec![],
        pos: 19
    };
}

#[test]
fn test_create_table() {
    assert_parse!("CREATE TABLE test (col)", create_table);
    assert_parse!("CREATE TABLE main.test (col)", create_table);
    assert_parse!(
        "CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL)",
        create_table
    );

    assert_parse!(
        "CREATE TABLE test (id INTERGER NOT NULL, PRIMARY KEY (id))",
        create_table
    );
    assert_parse!("CREATE TABLE test AS SELECT 1", create_table);

    assert_parse!("CREATE TEMP TABLE test (col)", create_table);
    assert_parse!("CREATE TABLE IF NOT EXISTS test (col)", create_table);

    fails_with! {
        parser: SqlParser,
        input: "CREATE TABLE test",
        rule: Rule::create_table,
        positives: vec![Rule::create_table_body],
        negatives: vec![],
        pos: 17
    };
    fails_with! {
        parser: SqlParser,
        input: "CREATE TABLE test ()",
        rule: Rule::create_table,
        positives: vec![Rule::column_def],
        negatives: vec![],
        pos: 19
    };
    fails_with! {
        parser: SqlParser,
        input: "CREATE TABLE test (PRIMARY KEY (id))",
        rule: Rule::create_table,
        positives: vec![Rule::signed_number],
        negatives: vec![],
        pos: 32
    };
    fails_with! {
        parser: SqlParser,
        input: "CREATE TABLE test (col,)",
        rule: Rule::create_table,
        positives: vec![Rule::column_def, Rule::table_constraint],
        negatives: vec![],
        pos: 23
    };
}

#[test]
fn test_column_definition() {
    assert_parse!("CREATE TABLE test (id UNSIGNED BIG INT)", create_table);
    assert_parse!("CREATE TABLE test (id INT8)", create_table);
    assert_parse!("CREATE TABLE test (id CHARACTER(20))", create_table);
    assert_parse!(
        "CREATE TABLE test (id VARYING CHARACTER(255))",
        create_table
    );
    assert_parse!("CREATE TABLE test (id DOUBLE PRECISION)", create_table);
    assert_parse!("CREATE TABLE test (id DECIMAL(10,5))", create_table);
}

#[test]
fn test_column_constraints() {
    assert_parse!(
        "CREATE TABLE test (id CONSTRAINT not_null NOT NULL)",
        create_table
    );
    assert_parse!(
        "CREATE TABLE test (id INTEGER PRIMARY KEY AUTOINCREMENT)",
        create_table
    );
    assert_parse!(
        "CREATE TABLE test (id INTEGER PRIMARY KEY ON CONFLICT IGNORE)",
        create_table
    );
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
    assert_parse!(
        "CREATE TABLE test (id, CONSTRAINT pk PRIMARY KEY (id))",
        create_table
    );
    assert_parse!("CREATE TABLE test (id, UNIQUE (id))", create_table);
    //assert_parse!("CREATE TABLE test (id, CHECK (id > 0))", create_table);
    assert_parse!(
        "CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable(id))",
        create_table
    );
    assert_parse!(
        "CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable)",
        create_table
    );
    assert_parse!(
        "CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable(id) DEFERRABLE \
         INITIALLY DEFERRED)",
        create_table
    );
}

#[test]
fn test_select() {
    assert_parse!("SELECT 1", cmd);
    assert_parse!("SELECT * FROM test ORDER BY 1", select);
    assert_parse!("SELECT * FROM test ORDER BY 1, id", select);
    assert_parse!("SELECT * FROM test LIMIT 1", select);
}

#[test]
fn test_one_select() {
    assert_parse!("SELECT 1", one_select);
    assert_parse!("SELECT 1, 'test'", one_select);

    assert_parse!("SELECT * FROM test WHERE 1", one_select);
    //assert_parse!("SELECT * FROM test WHERE 1 GROUP BY id HAVING count(*) > 1", one_select);

    /*fails_with! {
        parser: SqlParser,
        input: "SELECT 1 FROM WHERE 1",
        rule: Rule::one_select,
        positives: vec![],
        negatives: vec![],
        pos: 23
    };*/
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

#[test]
fn test_id() {
    assert_parse!("id", id);
}
