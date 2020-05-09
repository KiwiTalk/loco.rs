use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpAttachment {
    #[serde(rename = "Q")]
    pub question: String,
    #[serde(rename = "L")]
    pub link: String,
    #[serde(rename = "I")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "V")]
    pub resource_type: String, // TODO: what is `V`?
    #[serde(rename = "R")]
    pub resources: Vec<SharpResource>,
    #[serde(rename = "F")]
    pub footer: Option<SharpButtonList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpResource {
    kind: SharpResourceKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SharpResourceKind {
    SharpButtonList(SharpButtonList),
    SharpImage(SharpImage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpButtonList {
    #[serde(rename = "BU")]
    pub buttons: Vec<SharpButtonFragment>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpImage {
    #[serde(rename = "I")]
    pub image: Option<SharpImageFragment>,
    #[serde(rename = "L")]
    pub link: String,
    #[serde(rename = "T")]
    pub text: Option<SharpTextFragment>,
    #[serde(rename = "D")]
    pub description: Option<String>,
}

pub struct SharpFragment {
    kind: SharpFragmentKind,
}

pub enum SharpFragmentKind {
    Button(SharpButtonFragment),
    Image(SharpImageFragment),
    Text(SharpTextFragment),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpButtonFragment {
    #[serde(rename = "T")]
    pub text: String,
    #[serde(rename = "L")]
    pub link: Option<String>,
    #[serde(rename = "TP")]
    pub icon: Option<String>, // TODO: what is `TP`?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpImageFragment {
    #[serde(rename = "I")]
    pub url: String,
    #[serde(rename = "W")]
    pub width: Option<usize>,
    #[serde(rename = "H")]
    pub height: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharpTextFragment {
    #[serde(rename = "T")]
    pub text: String,
    #[serde(rename = "DE")]
    pub description: Option<String>,
}
