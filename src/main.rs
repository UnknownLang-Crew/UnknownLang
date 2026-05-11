mod unknown;
use crate::unknown::astprinter::AstPrinter;
use crate::unknown::lexer::Lexer;
use crate::unknown::parser::Parser;

fn main() {
    let source = "(1 + 2) * 3";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("{source}");
    AstPrinter::print(&ast);
}
