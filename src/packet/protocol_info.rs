#[derive(Debug)]
pub enum ProtocolInfo {
    GETCONF,
    BUYCS,
    NETTEST,
    CHECKIN,
    DOWN,
    MINI,
    COMPLETE,
    POST,
    SPOST,
    MPOST,
    ADDMEM,
    NEWMEM,
    LEAVE,
    DELMEM,
    LEFT,
    BLSYNC,
    BLADDITEM,
    BLDELITEM,
    BLSPAM,
    BLSPAMS,
    BLMEMBER,
    SHIP,
    MSHIP,
    GETTRAILER,
    INVOICE,
    MINVOICE,
    MCHKTOKENS,
    CREATE,
    PCREATE,
    CHATINFO,
    CHATOFF,
    CHATONROOM,
    GETMETA,
    SETMETA,
    CHGMETA,
    GETMETAS,
    GETMCMETA,
    SETMCMETA,
    CHGMCMETA,
    GETCHATST,
    SETCHATST,
    CHGCHATST,
    UPDATECHAT,
    GETMEM,
    MEMBER,
    WRITE,
    MSG,
    FORWARD,
    DECUNREAD,
    CLEARNOTI,
    CLRBDG,
    MCHATLOGS,
    SYNCMSG,
    DELETEMSG,
    SYNCDLMSG,
    SELFDLMSG,
    SELFDLAMSG,
    LOGINLIST,
    LCHATLIST,
    CHANGESVR,
    VOEVENT,
    SCREATE,
    SWRITE,
    SADDMEM,
    SETPK,
    SETSK,
    GETPK,
    GETSK,
    GETLPK,
    CREATELINK,
    DELETELINK,
    JOINLINK,
    JOININFO,
    INFOLINK,
    SYNCLINK,
    UPLINKPROF,
    KICKLEAVE,
    UPDATELINK,
    REPOLEAVE,
    SYNCMAINPF,
    SYNCLINKCR,
    SYNCLINKUP,
    SYNCLINKDL,
    KICKMEM,
    REPORTMEM,
    LINKKICKED,
    LNKDELETED,
    SYNCLINKPF,
    KICKED,
    SYNCJOIN,
    FEED,
    CHECKJOIN,
    BLIND,
    SYNCBLIND,
    REPORTLINK,
    KLSYNC,
    KLDELITEM,
    REACT,
    REACTCNT,
    SETMEMTYPE,
    SYNCMEMT,
    REWRITE,
    SYNCREWR,
    RELAYEVENT,
    SYNCEVENT,
    GRADD,
    GRADDSYNC,
    GRDEL,
    GRDELSYNC,
    GRUPDATE,
    GRUPSYNC,
    GRADDITEM,
    GRADDISYNC,
    GRDELITEM,
    GRDELISYNC,
    GRDELITEMA,
    GRDELIASYN,
    GRSETPOS,
    GRPOSSYNC,
    GRLIST,
    NOTIRCVS,
    CHGMOMETAS,
    GETMOMETA,
    MOCLICK,
    SETST,
    PUSHACK,
    SPUSH,
    GETTOKEN,
}