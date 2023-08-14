use full_moon::{
    ast::{FunctionArgs, FunctionCall, Prefix, Suffix},
    tokenizer::{Token, TokenReference, TokenType},
    visitors::VisitorMut,
    ShortString,
};

use crate::utils::{create_ident, create_string, FunctionCallExt, ValueExt};

#[derive(Debug, Clone, Default)]
pub struct RequirePatcher {
    pub requires: Vec<ShortString>,
}

impl RequirePatcher {
    const REQUIRE_IDENTIFIER: &str = "require";
    const BUNDLE_REQUIRE_IDENTIFIER: &str = "REQUIRE_MODULE";
}

impl VisitorMut for RequirePatcher {
    fn visit_function_call(&mut self, function_call: FunctionCall) -> FunctionCall {
        let is_require = function_call
            .name()
            .is_some_and(|name| name.as_str() == Self::REQUIRE_IDENTIFIER);

        let virtual_file_name = function_call.arg(0).and_then(|value| value.as_string());

        if let Some(virtual_file_name) = virtual_file_name.filter(|_| is_require) {
            let function_call = FunctionCall::new(Prefix::Name(TokenReference::new(
                vec![],
                create_ident(Self::BUNDLE_REQUIRE_IDENTIFIER),
                vec![Token::new(TokenType::Whitespace {
                    characters: ShortString::from(" "),
                })],
            )))
            .with_suffixes(vec![Suffix::Call(full_moon::ast::Call::AnonymousCall(
                FunctionArgs::String(TokenReference::new(
                    vec![],
                    create_string(virtual_file_name.as_str()),
                    vec![Token::new(TokenType::Whitespace {
                        characters: ShortString::from("\n"),
                    })],
                )),
            ))]);

            self.requires.push(virtual_file_name);

            function_call
        } else {
            function_call
        }
    }
}
