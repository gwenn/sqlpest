use pest::prelude::*;
use super::Rdp;

#[test]
fn test_select() {
    let mut parser = Rdp::new(StringInput::new("SELECT 1"));

    assert!(parser.cmd());
    assert!(parser.end());
}
