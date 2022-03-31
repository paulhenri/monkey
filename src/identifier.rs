use std::collections::HashMap;

use crate::token::{Token, TokenType};
pub struct TokenBuilder {
    keywords: HashMap<String, TokenType>,
}

impl TokenBuilder {
    pub fn default() -> TokenBuilder {
        let mut my_builder = TokenBuilder {
            keywords: HashMap::new(),
        };
        my_builder
            .keywords
            .insert("LET".to_string(), TokenType::LET);
        my_builder
            .keywords
            .insert("FN".to_string(), TokenType::FUNCTION);
        my_builder
            .keywords
            .insert("fn".to_string(), TokenType::FUNCTION);
        my_builder
            .keywords
            .insert("let".to_string(), TokenType::LET);

        return my_builder;
    }

    pub fn get_token_from_identifier(self, identifier: &String) -> Token {
        if let Some(toktype) = self.keywords.get(identifier).cloned() {
            let mytoken: Token = Token::new(toktype, identifier.clone());
            mytoken
        } else {
            let mytoken: Token = Token::new(TokenType::ILLEGAL, "".to_string());
            mytoken
        }
    }
}
