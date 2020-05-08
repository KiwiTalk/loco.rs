use reqwest::Url;
use crate::internal::Language;

pub fn get_emoticon_url(lang: &Language) -> Url {
    Url::parse(
        format!("http://item-{lang}.talk.kakao.co.kr", lang = lang.to_string()).as_ref()
    ).ok().unwrap()
}

pub fn get_emoticon_image_url(path: &str, lang: &Language) -> Url {
    let mut url = get_emoticon_url(lang);
    url.set_path(path);
    url
}

pub fn get_emoticon_title_url(id: &str, ext: &str, lang: &Language) -> Url {
    let mut url = get_emoticon_url(lang);
    url.set_path(format!("dw/{}.title.{}", id, ext).as_ref());
    url
}

pub fn get_emoticon_pack_url(id: &str, lang: &Language) -> Url {
    let mut url = get_emoticon_url(lang);
    url.set_path(format!("dw/{}.file.pack.zip", id).as_ref());
    url
}

pub fn get_emoticon_thumbnail_pack_url(id: &str, lang: &Language) -> Url {
    let mut url = get_emoticon_url(lang);
    url.set_path(format!("dw/{}.file.thumb_pack.zip", id).as_ref());
    url
}