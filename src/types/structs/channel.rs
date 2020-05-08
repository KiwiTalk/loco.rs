// Maps to ChannelMetaSetStruct
#[derive(Debug, Clone)]
pub struct ChannelMetaSet {
    channel_id: u64,
    meta_list: Vec<ChannelMeta>,
}

// Maps to ChannelMetaStruct
#[derive(Debug, Clone)]
pub struct ChannelMeta {
    channel_meta_type: ChannelMetaType,
    revision: u64,
    author_id: u64,
    content: String,
    updated_at: i32,
}

#[derive(Debug, Clone)]
pub enum ChannelMetaType {
    Notice = 1,
    Group = 2,
    Title = 3,
    Profile = 4,
    TV = 5,
    Privilege = 6,
    LiveTV = 7,
    PlusBackground = 8,
    LiveTalkInfo = 11,
    LiveTalkCount = 12,
}

// Maps to ChannelBoardMetaStruct
pub struct ChannelBoardMeta {}

pub enum ChannelBoardType {
    None = 0,
    FloatingNotice = 1,
    SideNotice = 2,
    Badge = 3,
}
