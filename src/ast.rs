#[derive(Debug)]
pub enum AstNode {
    Identifier(String),

    Variable(String),

    Number(f64),

    String(String),

    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },

    Let {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },

    Expression(Box<AstNode>),

    Statement(Box<AstNode>),
}
