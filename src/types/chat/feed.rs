pub struct ChatFeed {
    pub feed_type: FeedType,
}

pub enum FeedType {
    // -999999
    Undefined,
    // -1
    LocalLeave,
    // 1
    Invite,
    // 2
    Leave,
    // 3
    SecretLeave,
    // 4
    OpenlinkJoin,
    // 5
    OpenlinkDeleteLink,
    // 6
    OpenlinkKicked,
    // 7
    ChatKicked,
    // 8
    ChatDeleted,
    // 10
    RichContent,
    // 11
    OpenlinkStaffOn,
    // 12
    OpenlinkStaffOff,
    // 13
    OpenlinkRewriteFeed,
    // 14
    DeleteToAll,
    // 14
    OpenlinkHandOverHost,
}
