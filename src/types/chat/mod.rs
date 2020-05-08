mod attachment;
mod feed;

pub use attachment::*;
pub use feed::*;

pub struct Chat {
    pub kind: ChatKind,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TextChat {

}

#[derive(Debug, Clone)]
pub struct LongTextChat {

}

#[derive(Debug, Clone)]
pub struct PhotoChat {

}

#[derive(Debug, Clone)]
pub struct MultiPhotoChat {

}

#[derive(Debug, Clone)]
pub struct StaticEmoticonChat {

}

#[derive(Debug, Clone)]
pub struct AnimatedEmoticonChat {

}

#[derive(Debug, Clone)]
pub struct VideoChat {

}

#[derive(Debug, Clone)]
pub struct SharpSearchChat {

}

#[derive(Debug, Clone)]
pub struct ReplyChat {

}

#[derive(Debug, Clone)]
pub struct KakaoLinkV2Chat {

}
