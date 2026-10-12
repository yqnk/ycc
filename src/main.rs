use std::env;
use std::fs;
use std::path::Path;

use ycc::{AstPrettyPrinter, Parser, X86Backend};

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/4.c".to_string());
    let mut parser: Parser = Parser::init(&path);

    let program = parser.parse_program();

    let mut pretty_printer = AstPrettyPrinter::new();
    pretty_printer.pretty_print(&program);

    let mut backend = X86Backend::new();
    let asm = backend.emit_program(&program);

    let out_path = Path::new(&path).with_extension("s");
    fs::write(&out_path, &asm).expect("could not write assembly");

    println!("\nwrote {}", out_path.display());
}
