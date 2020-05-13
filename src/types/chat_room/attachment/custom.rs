use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomType {
    Feed,
    List,
    Commerce,
    Carousel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ButtonStyle {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ButtonDisplayTarget {
    #[serde(rename = "both")]
    All,
    #[serde(rename = "sender")]
    Sender,
    #[serde(rename = "receiver")]
    Receiver,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageCropStyle {
    Square = 0,
    None = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextFragment {
    #[serde(rename = "T")]
    text: String,
    #[serde(rename = "D")]
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkFragment {
    #[serde(rename = "LPC")]
    windows_link: String,
    #[serde(rename = "LMO")]
    macos_link: String,
    #[serde(rename = "LCA")]
    android_link: String,
    #[serde(rename = "LCI")]
    ios_link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageFragment {
    #[serde(rename = "THU")]
    url: String,
    #[serde(rename = "W")]
    width: u32,
    #[serde(rename = "H")]
    height: u32,
    #[serde(rename = "SC")]
    crop_style: ImageCropStyle,
    #[serde(rename = "LI")]
    is_live: bool,
    #[serde(rename = "PT")]
    playtime: u32,
}

#[derive(Debug, Clone)]
pub struct ButtonFragment {
    text: String,
    display_target: Option<ButtonDisplayTarget>,
    link: LinkFragment,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialFragment {
    #[serde(rename = "LK")]
    link: u32,
    #[serde(rename = "CM")]
    comment: u32,
    #[serde(rename = "SH")]
    share: u32,
    #[serde(rename = "VC")]
    view: u32,
    #[serde(rename = "SB")]
    subscriber: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileFragment {
    #[serde(rename = "TD")]
    description: TextFragment,
    #[serde(rename = "L")]
    link: LinkFragment,
    #[serde(rename = "BG")]
    background_image: ImageFragment,
    #[serde(rename = "TH")]
    thumbnail_image: ImageFragment,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomContent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomFeedContent {
    description: TextFragment,
    button_style: ButtonStyle,
    button_list: Vec<ButtonFragment>,
    thumbnail_list: Vec<ImageFragment>,
    thumbnail_count: u32,
    text_link: Option<LinkFragment>,
    full_text: bool,
    link: Option<LinkFragment>,
    profile: Option<ProfileFragment>,
    social: Option<SocialFragment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarouselCover {
    #[serde(rename = "TD")]
    text: TextFragment,
    #[serde(rename = "TH")]
    thumbnail_image: Option<ImageFragment>,
    #[serde(rename = "BG")]
    background_image: Option<ImageFragment>,
    #[serde(rename = "L")]
    link: Option<LinkFragment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomCarouselContent {
    card_type: CustomType,
    #[serde(rename = "CIL")]
    content_list: Vec<CustomContent>,
    #[serde(rename = "CHD")]
    content_head: Option<CarouselCover>,
    #[serde(rename = "CTA")]
    content_tail: Option<CarouselCover>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomInfo {
    #[serde(rename = "ME")]
    message: String,
    #[serde(rename = "TP")]
    custom_type: CustomType,
    #[serde(rename = "SID")]
    service_id: String,
    #[serde(rename = "DID")]
    provider_id: String,
    #[serde(rename = "VA")]
    android_version: String,
    #[serde(rename = "VI")]
    ios_version: String,
    #[serde(rename = "VW")]
    windows_version: String,
    #[serde(rename = "VM")]
    macos_version: String,
    #[serde(rename = "SNM")]
    service_nickname: Option<String>,
    #[serde(rename = "SIC")]
    service_icon: Option<String>,
    #[serde(rename = "SL")]
    service_link: Option<LinkFragment>,
    #[serde(rename = "L")]
    link: Option<LinkFragment>,
    #[serde(rename = "LOCK")]
    secure: Option<bool>,
    #[serde(rename = "FW")]
    fw: Option<bool>,
    #[serde(rename = "RF")]
    reference: String,
    #[serde(rename = "AD")]
    ad: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KakaoLinkInfo {
    #[serde(rename = "ai")]
    app_id: String,
    #[serde(rename = "ti")]
    template_id: Option<String>,
    #[serde(rename = "lv")]
    link_version: Option<String>,
    #[serde(rename = "ak")]
    app_key: Option<String>,
    #[serde(rename = "av")]
    app_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomAttachment {
    #[serde(rename = "P")]
    info: CustomInfo,
    #[serde(rename = "C")]
    content: Option<CustomContent>,
    #[serde(rename = "K")]
    link_info: Option<KakaoLinkInfo>,
}

impl<'de> Deserialize<'de> for ButtonFragment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ButtonFragmentRaw {
            #[serde(rename = "BU")]
            inner: ButtonFragmentRawInner,
            #[serde(rename = "L")]
            link: LinkFragment,
        }

        #[derive(Deserialize)]
        struct ButtonFragmentRawInner {
            #[serde(rename = "T")]
            text: String,
            #[serde(rename = "SR")]
            display_target: Option<ButtonDisplayTarget>,
        }

        let raw = ButtonFragmentRaw::deserialize(deserializer)?;
        Ok(Self {
            text: raw.inner.text,
            display_target: raw.inner.display_target,
            link: raw.link,
        })
    }
}

impl Serialize for ButtonFragment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        #[derive(Serialize)]
        struct ButtonFragmentRaw<'a> {
            #[serde(rename = "BU")]
            inner: ButtonFragmentRawInner<'a>,
            #[serde(rename = "L")]
            link: &'a LinkFragment,
        }

        #[derive(Serialize)]
        struct ButtonFragmentRawInner<'a> {
            #[serde(rename = "T")]
            text: &'a str,
            #[serde(rename = "SR")]
            display_target: &'a Option<ButtonDisplayTarget>,
        }

        let inner = ButtonFragmentRawInner {
            text: &self.text,
            display_target: &self.display_target,
        };

        let raw = ButtonFragmentRaw {
            inner,
            link: &self.link,
        };

        raw.serialize(serializer)
    }
}
