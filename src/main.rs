mod ast;
mod parser;

use parser::Parser;

fn main() {
    let code_example = r#"
        let x = 3;
        print("hello, world! my favorite number is $x");
    "#;

    let mut parser = Parser::new(code_example);

    while let Some(stmt) = parser.statement() {
        println!("{:?}", stmt);
    }
}
