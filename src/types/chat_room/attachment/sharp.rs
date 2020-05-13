use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpAttachment {
    #[serde(rename = "Q")]
    pub question: String,
    #[serde(rename = "L")]
    pub link: String,

    #[serde(rename = "I")]
    pub thumbnail_url: Option<String>,

    #[serde(rename = "V")]
    pub resource_type: String,
    // TODO: what is `V`?
    #[serde(rename = "R")]
    pub resources: Vec<resource::Resource>,

    #[serde(rename = "F")]
    pub footer: Option<resource::ButtonList>,
}

pub mod resource {
    use serde::{Deserialize, Serialize};

    use super::*;

    // TODO: will be integrated with `ResourceKind` in future.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum Type {
        None,
        List,
        Image,
        VideoClip,
        Weather,
        Movie,
        Media,
        Rank,
        Simple,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Resource {
        pub kind: ResourceKind,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum ResourceKind {
        ButtonList(ButtonList),
        Image(Image),
        Media(Media),
        Movie(Movie),
        Rank(Rank),
        Video(Video),
        Weather(Weather),
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ButtonList {
        #[serde(rename = "BU")]
        pub buttons: Vec<fragment::Button>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Image {
        #[serde(rename = "I")]
        pub image: Option<fragment::Image>,
        #[serde(rename = "L")]
        pub link: String,
        #[serde(rename = "T")]
        pub text: Option<fragment::Text>,
        #[serde(rename = "D")]
        pub description: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Media {
        #[serde(rename = "I")]
        pub image: Option<fragment::Image>,
        #[serde(rename = "L")]
        pub link: String,
        #[serde(rename = "T")]
        pub text: Option<fragment::Text>,
        #[serde(rename = "D")]
        pub description: Option<fragment::Text>,
        #[serde(rename = "DL")]
        pub details: Vec<fragment::Text>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Movie {
        #[serde(rename = "IL")]
        pub images: Vec<fragment::Image>,
        #[serde(rename = "L")]
        pub link: String,
        #[serde(rename = "T")]
        pub text: Option<fragment::Text>,
        #[serde(rename = "D")]
        pub description: Option<String>,
        #[serde(rename = "DL")]
        pub details: Vec<fragment::Text>,
        #[serde(rename = "ST")]
        pub stars: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Rank {
        #[serde(rename = "I")]
        pub image: Option<fragment::Image>,
        #[serde(rename = "L")]
        pub link: String,
        #[serde(rename = "T")]
        pub text: Option<fragment::Text>,
        #[serde(rename = "RA")]
        pub rank: Option<String>,
        #[serde(rename = "ST")]
        pub stars: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Video {
        #[serde(rename = "I")]
        pub image: Option<fragment::Image>,
        #[serde(rename = "L")]
        pub link: String,
        #[serde(rename = "T")]
        pub text: Option<fragment::Text>,
        #[serde(rename = "D")]
        pub description: Option<String>,
        #[serde(rename = "PT")]
        pub playtime: i32, // TODO: what is `playtime` format?
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Weather {
        #[serde(rename = "MA")]
        pub main_weather: Vec<fragment::Weather>,
        #[serde(rename = "SU")]
        pub sub_weather: Vec<fragment::Weather>,
        #[serde(rename = "L")]
        pub link: String,
        #[serde(rename = "PL")]
        pub place: Option<String>,
        #[serde(rename = "D")]
        pub description: Option<String>,
        #[serde(rename = "TM")]
        pub updated_at: String,
    }
}

pub mod fragment {
    use serde::{Deserialize, Serialize};

    #[allow(dead_code)]
    pub struct Fragment {
        kind: FragmentKind,
    }

    pub enum FragmentKind {
        Button(Button),
        Image(Image),
        Simple(Simple),
        Text(Text),
        Weather(Weather),
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Button {
        #[serde(rename = "T")]
        pub text: String,
        #[serde(rename = "L")]
        pub link: Option<String>,
        #[serde(rename = "TP")]
        pub icon: Option<String>, // TODO: what is `TP`?
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Image {
        #[serde(rename = "I")]
        pub url: String,
        #[serde(rename = "W")]
        pub width: Option<usize>,
        #[serde(rename = "H")]
        pub height: Option<usize>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Simple {
        #[serde(rename = "L")]
        pub link: String,
        #[serde(rename = "T")]
        pub text: Option<String>,
        #[serde(rename = "D")]
        pub description: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Text {
        #[serde(rename = "T")]
        pub text: String,
        #[serde(rename = "DE")]
        pub description: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Weather {
        #[serde(rename = "T")]
        pub text: Option<Text>,
        #[serde(rename = "TE")]
        pub temperature: String,
        #[serde(rename = "IC")]
        pub icon: WeatherIcon,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum WeatherIcon {
        Sunny = 0,
        CloudyDay = 3,
        CloudyNight = 2,
    }
}
