mod attachment;
mod feed;

pub use attachment::*;
pub use feed::*;

pub struct Chat {
    pub kind: ChatKind,
}

pub enum ChatKind {
    Text(TextChat),
    LongText(LongTextChat),
    Photo(PhotoChat),
    MultiPhoto(MultiPhotoChat),
    StaticEmoticon(StaticEmoticonChat),
    AnimatedEmoticon(AnimatedEmoticonChat),
    SharpSearch(SharpSearchChat),
    Reply(ReplyChat),
    KakaoLinkV2(KakaoLinkV2Chat),
}

pub struct TextChat {

}

pub struct LongTextChat {

}

pub struct PhotoChat {

}

pub struct MultiPhotoChat {

}

pub struct StaticEmoticonChat {

}

pub struct AnimatedEmoticonChat {

}

pub struct VideoChat {

}

pub struct SharpSearchChat {

}

pub struct ReplyChat {

}

pub struct KakaoLinkV2Chat {

}
