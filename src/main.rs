use std::env;

use ycc::ast::Parser;

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/1.c".to_string());
    let mut parser: Parser = Parser::init(&path);

    while !parser.is_eof() {
        println!("{:?}", parser.cur_token);
        parser.advance();
    }
}
