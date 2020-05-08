pub struct ChatFeed {
    pub feed_type: FeedType,
}

pub enum FeedType {
    Undefined = -999999,
    LocalLeave = -1,
    Invite = 1,
    Leave = 2,
    SecretLeave = 3,
    OpenLinkJoin = 4,
    OpenLinkDeleteLink = 5,
    OpenLinkKicked = 6,
    ChatKicked = 7,
    ChatDeleted = 8,
    RichContent = 10,
    OpenLinkStaffOn = 11,
    OpenLinkStaffOff = 12,
    OpenLinkRewriteFeed = 13,
    DeleteToAll = 14,
    OpenLinkHandOverHost = 15,
}
