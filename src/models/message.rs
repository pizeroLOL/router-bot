use serde::{Deserialize, Serialize};

use crate::models::basic_type::OneBotBool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageType {
    Flash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRecv {
    pub file: String,
    pub r#type: ImageType,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSend {
    pub file: String,
    pub r#type: ImageType,
    pub cache: OneBotBool,
    pub proxy: OneBotBool,
    // 文档没写
    /// 单位：秒
    pub timeout: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRecv {
    pub file: String,
    pub magic: OneBotBool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSend {
    pub file: String,
    pub magic: OneBotBool,
    pub url: String,
    pub cache: OneBotBool,
    pub proxy: OneBotBool,
    // 文档没写
    /// 单位：秒
    pub timeout: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRecv {
    pub file: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSend {
    pub file: String,
    pub cache: OneBotBool,
    pub proxy: OneBotBool,
    // 文档没写
    /// 单位：秒
    pub timeout: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AtQqType {
    #[serde(rename = "all")]
    All,
    Single(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct At {
    pub qq: AtQqType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rps {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shake {}

// TODO: Static Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokeReceive {
    pub r#type: isize,
    pub id: isize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokeRecv {
    pub r#type: isize,
    pub id: isize,
    pub name: String,
}

// pub struct AnonymousReceive {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousRecv {
    pub ignore: Option<OneBotBool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareReceive {
    pub url: String,
    pub title: String,
    pub content: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareRecv {
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactType {
    Group { id: String },
    Qq { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationRecv {
    pub lat: String,
    pub lon: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSend {
    pub lat: String,
    pub lon: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

// TODO: 转义
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Music {
    #[serde(rename = "163")]
    Netease {
        id: String,
    },
    Qq {
        id: String,
    },
    Custom {
        url: String,
        audio: String,
        title: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forward {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRecv {
    pub id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSend {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMergeForwardCq {
    pub user_id: String,
    pub nickname: String,
    pub content: CqMsg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMergeForwardJson {
    pub user_id: String,
    pub nickname: String,
    pub content: JsonMsgSend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xml {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Json {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum Send {
    Text(Text),
    Face(Face),
    Image(ImageSend),
    Record(RecordSend),
    Video(VideoSend),
    At(At),
    Rps(Rps),
    Dice(Dice),
    Shake(Shake),
    Poke(PokeRecv),
    Anonymous(AnonymousRecv),
    Share(ShareRecv),
    Contact(ContactType),
    Location(LocationSend),
    Music(Music),
    Reply(Reply),
    Forward(Forward),
    NodeSend(NodeSend),
    Xml(Xml),
    Json(Json),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum Recv {
    Text(Text),
    Face(Face),
    Image(ImageRecv),
    Record(RecordRecv),
    Video(VideoRecv),
    At(At),
    Rps(Rps),
    Dice(Dice),
    Shake(Shake),
    Poke(PokeRecv),
    Anonymous(AnonymousRecv),
    Share(ShareRecv),
    Contact(ContactType),
    Location(LocationRecv),
    Music(Music),
    Reply(Reply),
    Forward(Forward),
    NodeSend(NodeRecv),
    Xml(Xml),
    Json(Json),
}

// TODO: Serialize/Deserialize Cq
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CqMsg(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonMsgSend {
    Segment(Send),
    Array(Vec<Send>),
}

// TODO: JsonMsgSend into CqMsg

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonMsgRecv {
    Segment(Recv),
    Array(Vec<Recv>),
}
