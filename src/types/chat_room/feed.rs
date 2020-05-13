use serde::{Deserialize, Serialize};

use crate::types::Member;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feed {
    #[serde(rename = "feedType")]
    pub feed_type: FeedType,
    pub text: Option<String>,
    pub member: Option<Member>,
    pub members: Vec<Member>,
    pub inviter: Option<Member>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeedType {
    Undefined = -999_999,
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
