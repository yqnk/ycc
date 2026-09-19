use std::env;

use ycc::ast::{AstPrettyPrint, Parser};

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/1.c".to_string());
    let mut parser: Parser = Parser::init(&path);

    let program = parser.parse_program();
    program.print();
}
