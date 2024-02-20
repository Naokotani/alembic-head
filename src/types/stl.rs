use crate::types::content::{Content, Details, Summary};

pub struct Stl {
    pub content: Details,
    file: String,
}

impl Stl {
    pub fn new(content: Details, file: String) -> Self {
        Stl { content, file }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log;


    #[test_log::test]
    pub fn stl() {
        let content = Details::new(
            1,
            String::from("Cool Book"),
            String::from("thumb.jpg"),
            String::from("so great book"),
            String::from("Frank Hayes"),
        );
        let file = String::from("file.stl");
        Stl::new(content, file);
    }
}
