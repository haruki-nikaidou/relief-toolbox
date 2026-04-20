#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Variable(char),
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
}

pub enum ParseError {
    UnexpectedToken(Lixer),
    UnexpectedChar(char),
    UnexpectedEndOfInput,
    NoVariables,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lixer {
    Variable(char),
    Not,
    And,
    Or,
    Xor,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Lixer>, ParseError> {
    input
        .chars()
        .filter_map(|c| match c {
            c if c.is_whitespace() => None,
            'a'..='z' => Some(Ok(Lixer::Variable(c.to_ascii_uppercase()))),
            'A'..='Z' => Some(Ok(Lixer::Variable(c))),
            '!' => Some(Ok(Lixer::Not)),
            '&' => Some(Ok(Lixer::And)),
            '|' => Some(Ok(Lixer::Or)),
            '^' => Some(Ok(Lixer::Xor)),
            '(' => Some(Ok(Lixer::LParen)),
            ')' => Some(Ok(Lixer::RParen)),
            _ => Some(Err(ParseError::UnexpectedChar(c))),
        })
        .collect::<Result<Vec<Lixer>, ParseError>>()
}

pub struct Parser<'a> {
    tokens: &'a [Lixer],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Lixer]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<Lixer> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<Lixer> {
        let tok = self.tokens.get(self.pos).copied();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    // expr := or_expr
    fn parse_expr(&mut self) -> Result<Expression, ParseError> {
        self.parse_or()
    }

    // or_expr := xor_expr ('|' xor_expr)*
    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_xor()?;
        while matches!(self.peek(), Some(Lixer::Or)) {
            self.advance();
            let right = self.parse_xor()?;
            left = Expression::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    // xor_expr := and_expr ('^' and_expr)*
    fn parse_xor(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Some(Lixer::Xor)) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::Xor(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    // and_expr := not_expr ('&' not_expr)*
    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_not()?;
        while matches!(self.peek(), Some(Lixer::And)) {
            self.advance();
            let right = self.parse_not()?;
            left = Expression::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    // not_expr := '!' not_expr | atom
    fn parse_not(&mut self) -> Result<Expression, ParseError> {
        if matches!(self.peek(), Some(Lixer::Not)) {
            self.advance();
            let inner = self.parse_not()?; // right-assoc: !!A works
            Ok(Expression::Not(Box::new(inner)))
        } else {
            self.parse_atom()
        }
    }

    // atom := Variable | '(' expr ')'
    fn parse_atom(&mut self) -> Result<Expression, ParseError> {
        match self.advance() {
            Some(Lixer::Variable(c)) => Ok(Expression::Variable(c)),
            Some(Lixer::LParen) => {
                let inner = self.parse_expr()?;
                match self.advance() {
                    Some(Lixer::RParen) => Ok(inner),
                    Some(_) | None => Err(ParseError::UnexpectedEndOfInput),
                }
            }
            Some(_) => {
                // We advanced past an unexpected token; report it.
                // Recover the char from the token we just consumed:
                let tok = &self.tokens[self.pos - 1];
                Err(ParseError::UnexpectedToken(*tok))
            }
            None => Err(ParseError::UnexpectedEndOfInput),
        }
    }
}

pub fn parse(input: &str) -> Result<Expression, ParseError> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(&tokens);
    let expr = parser.parse_expr()?;
    if let Some(err) = parser.peek() {
        Err(ParseError::UnexpectedToken(err))
    } else {
        Ok(expr)
    }
}
