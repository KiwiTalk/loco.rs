// Maps to ClientSettingsStruct
#[derive(Debug, Clone)]
pub struct ClientSettings {
    status: i32,
    allow_pay: bool,
    allow_story: bool,
    allow_story_post: bool,
    background_image_url: String,
    original_background_image_url: String,
    profile_image_url: String,
    status_message: String,
    story_url: String,
    suspended: bool,
    user_id: u64,
}
