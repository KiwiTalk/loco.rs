pub enum CustomType {
    Feed,
    List,
    Commerce,
    Carousel,
}

pub enum ButtonStyle {
    Horizontal,
    Vertical,
}

pub enum ButtonDisplayTarget {
    All,
    Sender,
    Receiver,
}

pub enum ImageCropStyle {
    Square = 0,
    None = 1,
}

pub struct TextFragment {
    text: String,
    description: String,
}

pub struct LinkFragment {
    windows_link: String,
    macos_link: String,
    android_link: String,
    ios_link: String,
}

pub struct ImageFragment {
    url: String,
    width: u32,
    height: u32,
    crop_style: ImageCropStyle,
    is_live: bool,
    playtime: u32,
}

pub struct ButtonFragment {
    text: String,
    display_target: Option<ButtonDisplayTarget>,
    link: LinkFragment,
}

pub struct SocialFragment {
    link: u32,
    comment: u32,
    share: u32,
    view: u32,
    subscriber: u32,
}

pub struct ProfileFragment {
    description: TextFragment,
    link: LinkFragment,
    background_image: ImageFragment,
    thumbnail_image: ImageFragment,
}

pub struct CustomContent { }

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

pub struct CarouselCover {
    text: TextFragment,
    thumbnail_image: Option<ImageFragment>,
    background_image: Option<ImageFragment>,
    link: Option<LinkFragment>,
}

pub struct CustomCarouselContent {
    card_type: CustomType,
    content_list: Vec<CustomContent>,
    content_head: Option<CarouselCover>,
    content_tail: Option<CarouselCover>,
}

pub struct CustomInfo {
    message: String,
    custom_type: CustomType,
    service_id: String,
    provider_id: String,
    android_version: String,
    ios_version: String,
    windows_version: String,
    macos_version: String,
    service_nickname: Option<String>,
    service_icon: Option<String>,
    service_link: Option<LinkFragment>,
    link: Option<LinkFragment>,
    service: Option<bool>,
    fw: Option<bool>, // Its name should be changed.
    r#ref: String, // Its name must be changed!
    ad: Option<bool>, // Maybe is_ad?
}

pub struct KakaoLinkInfo {
    app_id: String,
    template_id: Option<String>,
    link_version: Option<String>,
    app_key: Option<String>,
    app_version: Option<String>,
}

pub struct CustomAttachment {
    info: CustomInfo,
    content: Option<CustomContent>,
    link_info: Option<KakaoLinkInfo>,
}