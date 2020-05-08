// Maps to ChannelMetaSetStruct
pub struct ChannelMetaSet {
    channel_id: u64,
    meta_list: Vec<ChannelMeta>,
}

// Maps to ChannelMetaStruct
pub struct ChannelMeta {
    channel_meta_type: ChannelMetaType,
    revision: u64,
    author_id: u64,
    content: String,
    updated_at: i32,
}

enum ChannelMetaType {
    // 1
    Notice,
    // 2
    Group,
    // 3
    Title,
    // 4
    Profile,
    // 5
    TV,
    // 6
    Privilege,
    // 7
    LiveTV,
    // 8
    PlusBackground,
    // 11
    LiveTalkInfo,
    // 12
    LiveTalkCount,
}

// Maps to ChannelBoardMetaStruct
pub struct ChannelBoardMeta {

}

enum ChannelBoardType {
    // 0
    None,
    // 1
    FloatingNotice,
    // 2
    SideNotice,
    // 3
    Badge,
}