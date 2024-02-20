use crate::types::content::{Details, Summary, Content, Dimensions};


pub struct Token {
    pub content: Details,
    pub dimensions: Dimensions,
    pub image: String,
}

impl Token {
    pub fn new(content: Details, dimensions: Dimensions, image: String) -> Self {
        Token {
            content,
            dimensions,
            image,
        }
    }
}

pub struct TokenPack {
    content: Details,
    tokens: Vec<Token>,
}

impl TokenPack {
    pub fn new(content: Details, tokens: Vec<Token>) -> Self {
        TokenPack { content, tokens }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_log;

    #[test_log::test]
    pub fn token_pack() {
        let content = Details::new(
            1,
            String::from("Cool Book"),
            String::from("thumb.jpg"),
            String::from("so great book"),
            String::from("Frank Hayes"),
        );

        let content_token = Details::new(
            1,
            String::from("Cool Book"),
            String::from("thumb.jpg"),
            String::from("so great book"),
            String::from("Frank Hayes"),
        );
        let token = Token::new(
            content_token,
            Dimensions {
                width: 500,
                height: 500,
            },
            String::from("token.jpg"),
        );
        let tokens = vec![token];
        let token_pack = TokenPack::new(content, tokens);

        assert_eq!(token_pack.content.title, "Cool Book");
        assert_eq!(token_pack.content.thumb, "thumb.jpg");
        assert_eq!(token_pack.content.author, "Frank Hayes");
        assert_eq!(token_pack.content.summary, "so great book");
        assert_eq!(token_pack.tokens[0].image, "token.jpg");
        assert_eq!(
            token_pack.tokens[0].dimensions,
            Dimensions {
                width: 500,
                height: 500
            }
        );
    }
}
