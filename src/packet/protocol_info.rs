use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProtocolInfo {
    #[serde(rename = "GETCONF")]
    GetConfig,
    BuyCS, // TODO: What is CS?
    #[serde(rename = "NETTEST")]
    NetworkTest,
    CheckIn,
    Down,
    Mini,
    Complete,
    Post,
    SPost,
    MPost,
    #[serde(rename = "ADDMEM")]
    AddMember,
    #[serde(rename = "NEWMEM")]
    NewMember,
    Leave,
    #[serde(rename = "DELMEM")]
    DeleteMember,
    Left,
    #[serde(rename = "BLSYNC")]
    BlockSync,
    #[serde(rename = "BLADDITEM")]
    BlockAddItem,
    #[serde(rename = "BLDELITEM")]
    BlockDeleteItem,
    #[serde(rename = "BLSPAM")]
    BlockSpam,
    #[serde(rename = "BLSPAMS")]
    BlockSpams,
    #[serde(rename = "BLMEMBER")]
    BlockMember,
    Ship,
    MShip, // TODO: What is M?
    GetTrailer,
    Invoice,
    MInvoice, // TODO: What is M?
    #[serde(rename = "MCHKTOKENS")]
    MCheckTokens,
    Create,
    PCreate,
    ChatInfo,
    ChatOff,
    ChatOnRoom,
    GetMeta,
    SetMeta,
    #[serde(rename = "CHGMETA")]
    ChangeMeta,
    GetMetas,
    GetMCMeta,
    SetMCMeta,
    #[serde(rename = "CHGMCMETA")]
    ChangeMCMeta,
    GetChatST,
    SetChatST,
    #[serde(rename = "CHGCHATST")]
    ChangeChatST,
    UpdateChat,
    #[serde(rename = "GETMEM")]
    GetMember,
    Member,
    Write,
    #[serde(rename = "MSG")]
    Message,
    Forward,
    #[serde(rename = "DECUNREAD")]
    DecreaseUnread,
    #[serde(rename = "CLEARNOTI")]
    ClearNotification,
    #[serde(rename = "CLRBDG")]
    ClearBadge,
    MChatLogs,
    #[serde(rename = "SYNCMSG")]
    SyncMessage,
    #[serde(rename = "DELETEMSG")]
    DeleteMessage,
    #[serde(rename = "SYNCDLMSG")]
    SyncDeleteMessage,
    #[serde(rename = "SELFDLMSG")]
    SelfDeleteMessage,
    #[serde(rename = "SELFDLAMSG")]
    SelfDLAMessage,
    LoginList,
    #[serde(rename = "LCHATLIST")]
    LoginChatList,
    #[serde(rename = "CHANGESVR")]
    ChangeServer,
    VOEvent,
    SCreate,
    SWrite,
    #[serde(rename = "SADDMEM")]
    SAddMember,
    #[serde(rename = "SETPK")]
    SetPublicKey,
    #[serde(rename = "SETSK")]
    SetSecretKey,
    #[serde(rename = "GETPK")]
    GetPublicKey,
    #[serde(rename = "GETSK")]
    GetSecretKey,
    #[serde(rename = "GETLPK")]
    GetLdapPublicKey,
    CreateLink,
    DeleteLink,
    JoinLink,
    JoinInfo,
    InfoLink,
    SyncLink,
    #[serde(rename = "UPLINKPROF")]
    UpdateLinkProfile,
    KickLeave,
    UpdateLink,
    RepoLeave,
    #[serde(rename = "SYNCMAINPF")]
    SyncMainProfile,
    SyncLinkCR,
    #[serde(rename = "SYNCLINKUP")]
    SyncLinkUpdate,
    #[serde(rename = "SYNCLINKDL")]
    SyncLinkDownload,
    #[serde(rename = "KICKMEM")]
    KickMember,
    #[serde(rename = "REPORTMEM")]
    ReportMember,
    LinkKicked,
    #[serde(rename = "LNKDELETED")]
    LinkDeleted,
    #[serde(rename = "SYNCMAINPF")]
    SyncLinkProfile,
    Kicked,
    SyncJoin,
    Feed,
    CheckJoin,
    Blind,
    SyncBlind,
    ReportLink,
    KLSync, // TODO: What is KL?
    #[serde(rename = "KLDELITEM")]
    KLDeleteItem,
    React,
    #[serde(rename = "REACTCNT")]
    ReactCount,
    #[serde(rename = "SETMEMTYPE")]
    SetMemberType,
    #[serde(rename = "SYNCMEMT")]
    SyncMemberType,
    Rewrite,
    #[serde(rename = "SYNCREWR")]
    SyncRewrite,
    RelayEvent,
    SyncEvent,
    #[serde(rename = "GRADD")]
    GroupAdd,
    #[serde(rename = "GRADDSYNC")]
    GroupAddSync,
    #[serde(rename = "GRDEL")]
    GroupDelete,
    #[serde(rename = "GRDELSYNC")]
    GroupDeleteSync,
    #[serde(rename = "GRUPDATE")]
    GroupUpdate,
    #[serde(rename = "GRUPSYNC")]
    GroupUpdateSync,
    #[serde(rename = "GRADDITEM")]
    GroupAddItem,
    #[serde(rename = "GRADDISYNC")]
    GroupAddItemSync,
    #[serde(rename = "GRDELITEM")]
    GroupDeleteItem,
    #[serde(rename = "GRDELISYNC")]
    GroupDeleteItemSync,
    #[serde(rename = "GRDELITEMA")]
    GroupDeleteItemAttr,
    #[serde(rename = "GRDELIASYN")]
    GroupDeleteItemAttrSync,
    #[serde(rename = "GRSETPOS")]
    GroupSetPosition,
    #[serde(rename = "GRPOSSYNC")]
    GroupPositionSync,
    #[serde(rename = "GRLIST")]
    GroupList,
    #[serde(rename = "NOTIRCVS")]
    NotificationReceiveSync,
    #[serde(rename = "CHGMOMETAS")]
    ChangeMoimMetas,
    #[serde(rename = "GETMOMETA")]
    GetMoimMeta,
    #[serde(rename = "MOCLICK")]
    MoimClick,
    SetST, // TODO: What is ST?
    PushAck,
    SPush, // TODO: What is S?
    GetToken,
}
