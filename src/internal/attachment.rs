use reqwest::Url;

use crate::define_host;

define_host!(profile_upload, "up-p.talk.kakao.com");
define_host!(media_upload, "up-gp.talk.kakao.com");
define_host!(video_upload, "up-m.talk.kakao.com");
define_host!(audio_upload, "up-v.talk.kakao.com");
define_host!(group_profile_upload, "up-a.talk.kakao.com");
define_host!(file, "up-p.talk.kakao.com");
define_host!(media_file, "up-p.talk.kakao.com");
define_host!(audio_file, "up-p.talk.kakao.com");
define_host!(video_file, "up-p.talk.kakao.com");

pub enum AttachmentType {
    // image/jpeg
    Image,
    // audio/mp4
    Audio,
    // video/mp4
    Video,
    // image/jpeg, (application/*)
    File,
}

impl AttachmentType {
    pub fn get_upload_url(&self) -> Url {
        match self {
            AttachmentType::Image => media_upload::url(),
            AttachmentType::Audio => audio_upload::url(),
            AttachmentType::Video => video_upload::url(),
            _ => media_upload::url(),
        }
    }

    pub fn get_attachment_url(&self) -> Url {
        match self {
            AttachmentType::Image => media_file::url(),
            AttachmentType::Audio => audio_file::url(),
            AttachmentType::Video => video_file::url(),
            AttachmentType::File => media_file::url(),
            /*_ => media_file::URL, */
        }
    }

    pub fn get_uploaded_file_url(&self, upload_path: &str) -> Url {
        let mut url: Url = self.get_attachment_url();
        url.set_path(upload_path);
        url
    }
}
