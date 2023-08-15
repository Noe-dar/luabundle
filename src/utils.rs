use std::io::{self, Write};

use full_moon::{
    ast::{Call, Expression, FunctionArgs, FunctionCall, Prefix, Suffix, Value},
    tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType},
    ShortString,
};

pub fn create_string<S: Into<String> + AsRef<str>>(string: S) -> Token {
    Token::new(TokenType::StringLiteral {
        literal: ShortString::new(string),
        multi_line: None,
        quote_type: StringLiteralQuoteType::Double,
    })
}

pub fn create_ident<S: Into<String> + AsRef<str>>(string: S) -> Token {
    Token::new(TokenType::Identifier {
        identifier: ShortString::new(string),
    })
}
pub trait WriteExt: Write {
    fn writeln_str<S: AsRef<str> + ?Sized>(&mut self, string: &S) -> io::Result<()> {
        let bytes = string.as_ref().as_bytes();
        self.write_all(bytes)?;
        self.write_all("\n".as_bytes())?;

        Ok(())
    }
}

impl<W: Write> WriteExt for W {}

pub trait FunctionCallExt {
    fn name(&self) -> Option<ShortString>;
    fn arg(&self, n: usize) -> Option<Value>;
}

impl FunctionCallExt for FunctionCall {
    fn arg(&self, n: usize) -> Option<Value> {
        if let Some(Suffix::Call(Call::AnonymousCall(anonymous_call))) = self.suffixes().next() {
            match anonymous_call {
                FunctionArgs::Parentheses { arguments, .. } => {
                    let argument = arguments.iter().nth(n);
                    let argument = argument.and_then(|value| value.as_value());
                    argument.map(|value| value.to_owned())
                }
                FunctionArgs::String(string) => Some(Value::String(string.to_owned())),
                _ => None,
            }
        } else {
            None
        }
    }

    fn name(&self) -> Option<ShortString> {
        match self.prefix() {
            Prefix::Name(name) => name.as_identifier(),
            _ => None,
        }
    }
}

pub trait ExpressionExt {
    fn as_value(&self) -> Option<&Value>;
}

impl ExpressionExt for Expression {
    fn as_value(&self) -> Option<&Value> {
        match self {
            Expression::Value { value } => Some(value),
            _ => None,
        }
    }
}

pub trait ValueExt {
    fn as_string(&self) -> Option<ShortString>;
}

impl ValueExt for Value {
    fn as_string(&self) -> Option<ShortString> {
        match self {
            Value::String(string) => string.as_string(),
            _ => None,
        }
    }
}

pub trait TokenReferenceExt {
    fn as_string(&self) -> Option<ShortString>;
    fn as_identifier(&self) -> Option<ShortString>;
}

impl<'token> TokenReferenceExt for &'token TokenReference {
    fn as_string(&self) -> Option<ShortString> {
        match self.token().token_type() {
            TokenType::StringLiteral { literal, .. } => Some(literal.clone()),
            _ => None,
        }
    }

    fn as_identifier(&self) -> Option<ShortString> {
        match self.token().token_type() {
            TokenType::Identifier { identifier } => Some(identifier.clone()),
            _ => None,
        }
    }
}
