#[derive(Debug, PartialEq)]
pub enum DisplayName {
    Name,
    Other,
    NamePublisher,
    OtherPublisher,
}

impl DisplayName {
    pub fn store(&self) -> &str {
        match self {
            Self::Name => "name",
            Self::Other => "other",
            Self::NamePublisher => "name_publisher",
            Self::OtherPublisher => "other_publisher",
        }
    }

    pub fn retreieve(str: &str) -> Self {
        match str {
            "name" => Self::Name,
            "other" => Self::Other,
            "name_publisher" => Self::NamePublisher,
            "other_publisher" => Self::OtherPublisher,
            _ => panic!("Invalid DisplayName string")
        }
    }
}
