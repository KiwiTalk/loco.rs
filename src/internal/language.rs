pub enum Language {
    Korean,
    Raw(String),
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Korean => "ko".to_string(),
            Language::Raw(language) => language.to_owned(),
        }
    }
}
