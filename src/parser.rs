use super::ast::AstNode;

use std::{iter::Peekable, str::Chars};

pub struct Parser<'a> {
    code_chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code_chars: code.chars().peekable(),
        }
    }

    fn expect_char(&mut self, c: char) -> Option<char> {
        if *(self.code_chars.peek()?) == c {
            self.code_chars.next()
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.code_chars
            .peek()
            .map(|&c| c.is_whitespace())
            .unwrap_or(false)
        {
            self.code_chars.next().unwrap();
        }
    }

    fn identifier(&mut self) -> Option<AstNode> {
        let mut identifier = String::new();

        loop {
            match self.code_chars.peek() {
                Some(c) if c.is_ascii_alphabetic() => {}
                _ => break,
            }

            identifier.push(self.code_chars.next().unwrap());
        }

        if identifier.is_empty() {
            None
        } else {
            Some(AstNode::Identifier(identifier))
        }
    }

    fn assignment(&mut self) -> Option<AstNode> {
        self.skip_whitespace();
        let lhs = self.identifier()?;
        self.skip_whitespace();
        self.expect_char('=')?;
        self.skip_whitespace();
        let rhs = self.expression()?;

        Some(AstNode::Let {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn number(&mut self) -> Option<AstNode> {
        let mut buf = String::new();

        loop {
            match self.code_chars.peek() {
                Some(c) => {
                    buf.push(*c);

                    if buf.parse::<f64>().is_err() {
                        buf.pop();
                        break;
                    }
                }

                None => break,
            }

            self.code_chars.next().unwrap();
        }

        if buf.is_empty() {
            None
        } else {
            Some(AstNode::Number(buf.parse().unwrap()))
        }
    }

    fn string(&mut self) -> Option<AstNode> {
        // TODO: Handle escapes.

        self.expect_char('"')?;

        let mut buf = String::new();

        while self.expect_char('"').is_none() {
            if let Some(c) = self.code_chars.next() {
                buf.push(c);
            } else {
                break;
            }
        }

        // the while condition takes care of the other "

        Some(AstNode::String(buf))
    }

    fn expression(&mut self) -> Option<AstNode> {
        self.skip_whitespace();

        if let Some(AstNode::Identifier(identifier)) = self.identifier() {
            match &identifier as &str {
                "let" => Some(AstNode::Expression(Box::new(self.assignment()?))),

                _ => {
                    self.skip_whitespace();

                    if let Some('(') = self.expect_char('(') {
                        // function call
                        let mut args = Vec::new();

                        loop {
                            match self.expression() {
                                Some(arg) => args.push(arg),
                                None => break,
                            }

                            self.skip_whitespace();
                            if self.expect_char(',').is_none() {
                                break;
                            }
                            self.skip_whitespace();
                        }

                        self.expect_char(')')?;

                        Some(AstNode::Expression(Box::new(AstNode::FunctionCall {
                            name: identifier,
                            args,
                        })))
                    } else {
                        // variable lookup
                        Some(AstNode::Expression(Box::new(AstNode::Variable(identifier))))
                    }
                }
            }
        } else if let Some(n) = self.number() {
            Some(AstNode::Expression(Box::new(n)))
        } else if let Some(s) = self.string() {
            Some(AstNode::Expression(Box::new(s)))
        } else {
            None
        }
    }

    pub fn statement(&mut self) -> Option<AstNode> {
        let expression = self.expression()?;
        self.skip_whitespace();
        self.expect_char(';')?;
        Some(AstNode::Statement(Box::new(expression)))
    }
}
