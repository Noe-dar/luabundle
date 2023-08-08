use full_moon::{
    tokenizer::{Token, TokenType},
    visitors::VisitorMut,
};

#[derive(Default, Clone, Copy, Debug)]
pub struct CommentPatcher;

impl VisitorMut for CommentPatcher {
    fn visit_single_line_comment(&mut self, _: Token) -> Token {
        Token::new(TokenType::Eof)
    }

    fn visit_multi_line_comment(&mut self, _: Token) -> Token {
        Token::new(TokenType::Eof)
    }
}
