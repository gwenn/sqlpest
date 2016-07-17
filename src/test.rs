use pest::prelude::*;
use super::Rdp;

#[test]
fn test_select() {
    let mut parser = Rdp::new(StringInput::new("SELECT 1"));

    assert!(parser.cmd());
    assert!(parser.end());
}

#[test]
fn test_one_select() {
    let mut parser = Rdp::new(StringInput::new("SELECT 1"));

    assert!(parser.one_select());
    assert!(parser.end());
}

#[test]
fn test_select_column() {
    let mut parser = Rdp::new(StringInput::new("1"));

    assert!(parser.select_column());
    assert!(parser.end());
}

#[test]
fn test_expr() {
    let mut parser = Rdp::new(StringInput::new("1"));

    assert!(parser.expr());
    assert!(parser.end());
}

#[test]
fn test_number() {
    let mut parser = Rdp::new(StringInput::new("1"));

    assert!(parser.number());
    assert!(parser.end());
}
