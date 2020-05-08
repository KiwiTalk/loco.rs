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
    OpenLinkJoin,
    // 5
    OpenLinkDeleteLink,
    // 6
    OpenLinkKicked,
    // 7
    ChatKicked,
    // 8
    ChatDeleted,
    // 10
    RichContent,
    // 11
    OpenLinkStaffOn,
    // 12
    OpenLinkStaffOff,
    // 13
    OpenLinkRewriteFeed,
    // 14
    DeleteToAll,
    // 14
    OpenLinkHandOverHost,
}
