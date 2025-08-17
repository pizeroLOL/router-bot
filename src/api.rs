use serde::{Deserialize, Serialize};

use crate::{
    adapters::http::{CqReq, Entry, JsonReq},
    models::{
        basic_type::{GroupRole, Sex},
        event::Sender,
        message::{CqMsg, JsonMsgRecv, JsonMsgSend, Recv},
    },
};

#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    pub message_id: i32,
}

#[derive(Debug, Serialize)]
pub struct SendMessageRequestCq {
    pub user_id: i64,
    pub message: CqMsg,
    pub auto_escape: bool,
}
impl Entry for SendMessageRequestCq {
    const ENTRY: &'static str = "send_private_msg";
    type Output = SendMessageResponse;
}
impl CqReq for SendMessageRequestCq {}

#[derive(Debug, Serialize)]
pub struct SendMessageRequestJson {
    pub user_id: i64,
    pub message: JsonMsgSend,
    pub auto_escape: bool,
}
impl Entry for SendMessageRequestJson {
    const ENTRY: &'static str = "send_private_msg";
    type Output = SendMessageResponse;
}
impl JsonReq for SendMessageRequestJson {}

#[derive(Debug, Deserialize)]
pub struct SendGroupMessageResponse {
    pub message_id: i32,
}

#[derive(Debug, Serialize)]
pub struct SendGroupMessageRequestCq {
    pub group_id: i64,
    pub message: CqMsg,
    pub auto_escape: bool,
}
impl Entry for SendGroupMessageRequestCq {
    const ENTRY: &'static str = "send_group_msg";
    type Output = SendGroupMessageResponse;
}
impl CqReq for SendGroupMessageRequestCq {}

#[derive(Debug, Serialize)]
pub struct SendGroupMessageRequestJson {
    pub group_id: i64,
    pub message: JsonMsgSend,
    pub auto_escape: bool,
}
impl Entry for SendGroupMessageRequestJson {
    const ENTRY: &'static str = "send_group_msg";
    type Output = SendGroupMessageResponse;
}
impl JsonReq for SendGroupMessageRequestJson {}

#[derive(Debug, Deserialize)]
pub struct SendMsgResponse {
    pub message_id: i32,
}

#[derive(Debug, Serialize)]
pub enum MessageType {
    Private,
    Group,
}

#[derive(Debug, Serialize)]
pub struct SendMsgRequestCq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<MessageType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    pub message: CqMsg,
    #[serde(default)]
    pub auto_escape: bool,
}
impl Entry for SendMsgRequestCq {
    const ENTRY: &'static str = "send_msg";
    type Output = SendMsgResponse;
}
impl CqReq for SendMsgRequestCq {}

#[derive(Debug, Serialize)]
pub struct SendMsgRequestJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<MessageType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    pub message: JsonMsgSend,
    #[serde(default)]
    pub auto_escape: bool,
}
impl Entry for SendMsgRequestJson {
    const ENTRY: &'static str = "send_msg";
    type Output = SendMsgResponse;
}
impl JsonReq for SendMsgRequestJson {}

#[derive(Debug, Deserialize)]
pub struct Empty {}

#[derive(Debug, Serialize)]
pub struct DeleteMessageRequest {
    pub message_id: i32,
}
impl Entry for DeleteMessageRequest {
    const ENTRY: &'static str = "delete_msg";
    type Output = Empty;
}
impl CqReq for DeleteMessageRequest {}
impl JsonReq for DeleteMessageRequest {}

#[derive(Debug, Serialize)]
pub struct GetMessageRequest {
    pub message_id: i32,
}
impl Entry for GetMessageRequest {
    const ENTRY: &'static str = "get_msg";
    type Output = GetMessageResponse;
}
impl CqReq for GetMessageRequest {}
impl JsonReq for GetMessageRequest {}

#[derive(Debug, Deserialize)]
pub struct GetMessageResponse {
    pub time: i32,
    pub message_type: String,
    pub message_id: i32,
    pub real_id: i32,
    pub sender: Sender,
    pub message: JsonMsgRecv,
}

#[derive(Debug, Serialize)]
pub struct GetForwardMessageRequest {
    pub id: String,
}
impl Entry for GetForwardMessageRequest {
    const ENTRY: &'static str = "get_forward_msg";
    type Output = GetForwardMessageResponse;
}
impl CqReq for GetForwardMessageRequest {}
impl JsonReq for GetForwardMessageRequest {}

#[derive(Debug, Deserialize)]
pub struct GetForwardMessageResponse {
    pub messages: Vec<Recv>,
}

#[derive(Debug, Serialize)]
pub struct SendLikeRequest {
    pub user_id: i64,
    #[serde(default = "default_times")]
    pub times: i32,
}

#[allow(dead_code)]
fn default_times() -> i32 {
    1
}

impl Entry for SendLikeRequest {
    const ENTRY: &'static str = "send_like";
    type Output = Empty;
}

impl CqReq for SendLikeRequest {}
impl JsonReq for SendLikeRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupKickRequest {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(default)]
    pub reject_add_request: bool,
}

impl Entry for SetGroupKickRequest {
    const ENTRY: &'static str = "set_group_kick";
    type Output = Empty;
}

impl CqReq for SetGroupKickRequest {}
impl JsonReq for SetGroupKickRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupBanRequest {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(default = "default_ban_duration")]
    pub duration: i32,
}

#[allow(dead_code)]
fn default_ban_duration() -> i32 {
    30 * 60
}

impl Entry for SetGroupBanRequest {
    const ENTRY: &'static str = "set_group_ban";
    type Output = Empty;
}

impl CqReq for SetGroupBanRequest {}
impl JsonReq for SetGroupBanRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupAnonymousBanRequest {
    pub group_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(default = "default_ban_duration")]
    pub duration: i32,
}

impl Entry for SetGroupAnonymousBanRequest {
    const ENTRY: &'static str = "set_group_anonymous_ban";
    type Output = Empty;
}

impl CqReq for SetGroupAnonymousBanRequest {}
impl JsonReq for SetGroupAnonymousBanRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupWholeBanRequest {
    pub group_id: i64,
    #[serde(default = "default_whole_ban_enable")]
    pub enable: bool,
}

#[allow(dead_code)]
fn default_whole_ban_enable() -> bool {
    true
}

impl Entry for SetGroupWholeBanRequest {
    const ENTRY: &'static str = "set_group_whole_ban";
    type Output = Empty;
}

impl CqReq for SetGroupWholeBanRequest {}
impl JsonReq for SetGroupWholeBanRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupAdminRequest {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(default = "default_admin_enable")]
    pub enable: bool,
}

#[allow(dead_code)]
fn default_admin_enable() -> bool {
    true
}

impl Entry for SetGroupAdminRequest {
    const ENTRY: &'static str = "set_group_admin";
    type Output = Empty;
}

impl CqReq for SetGroupAdminRequest {}
impl JsonReq for SetGroupAdminRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupCardRequest {
    pub group_id: i64,
    pub user_id: i64,
    pub card: String,
}
impl Entry for SetGroupCardRequest {
    const ENTRY: &'static str = "set_group_card";
    type Output = Empty;
}
impl CqReq for SetGroupCardRequest {}
impl JsonReq for SetGroupCardRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupNameRequest {
    pub group_id: i64,
    pub group_name: String,
}
impl Entry for SetGroupNameRequest {
    const ENTRY: &'static str = "set_group_name";
    type Output = Empty;
}
impl CqReq for SetGroupNameRequest {}
impl JsonReq for SetGroupNameRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupLeaveRequest {
    pub group_id: i64,
    #[serde(default)]
    pub is_dismiss: bool,
}
impl Entry for SetGroupLeaveRequest {
    const ENTRY: &'static str = "set_group_leave";
    type Output = Empty;
}
impl CqReq for SetGroupLeaveRequest {}
impl JsonReq for SetGroupLeaveRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupSpecialTitleRequest {
    pub group_id: i64,
    pub user_id: i64,
    pub special_title: String,
    #[serde(default = "default_special_title_duration")]
    pub duration: i32,
}

#[allow(dead_code)]
fn default_special_title_duration() -> i32 {
    -1
}

impl Entry for SetGroupSpecialTitleRequest {
    const ENTRY: &'static str = "set_group_special_title";
    type Output = Empty;
}
impl CqReq for SetGroupSpecialTitleRequest {}
impl JsonReq for SetGroupSpecialTitleRequest {}

#[derive(Debug, Serialize)]
pub struct SetFriendAddRequest {
    pub flag: String,
    #[serde(default)]
    pub approve: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}
impl Entry for SetFriendAddRequest {
    const ENTRY: &'static str = "set_friend_add_request";
    type Output = Empty;
}
impl CqReq for SetFriendAddRequest {}
impl JsonReq for SetFriendAddRequest {}

#[derive(Debug, Serialize)]
pub struct SetGroupAddRequest {
    pub flag: String,
    #[serde(rename = "type")]
    pub sub_type: String,
    #[serde(default)]
    pub approve: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl Entry for SetGroupAddRequest {
    const ENTRY: &'static str = "set_group_add_request";
    type Output = Empty;
}
impl CqReq for SetGroupAddRequest {}
impl JsonReq for SetGroupAddRequest {}

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}

#[derive(Debug, Serialize)]
pub struct GetLoginInfoRequest {}
impl Entry for GetLoginInfoRequest {
    const ENTRY: &'static str = "get_login_info";
    type Output = LoginInfo;
}
impl CqReq for GetLoginInfoRequest {}
impl JsonReq for GetLoginInfoRequest {}

#[derive(Debug, Deserialize)]
pub struct StrangerInfo {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Sex,
    pub age: i32,
}

#[derive(Debug, Serialize)]
pub struct GetStrangerInfoRequest {
    pub user_id: i64,
    #[serde(default)]
    pub no_cache: bool,
}
impl Entry for GetStrangerInfoRequest {
    const ENTRY: &'static str = "get_stranger_info";
    type Output = StrangerInfo;
}
impl CqReq for GetStrangerInfoRequest {}
impl JsonReq for GetStrangerInfoRequest {}

#[derive(Debug, Deserialize)]
pub struct Friend {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}

#[derive(Debug, Serialize)]
pub struct GetFriendListRequest {}
impl Entry for GetFriendListRequest {
    const ENTRY: &'static str = "get_friend_list";
    type Output = Vec<Friend>;
}
impl CqReq for GetFriendListRequest {}
impl JsonReq for GetFriendListRequest {}

#[derive(Debug, Deserialize)]
pub struct GroupInfo {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

#[derive(Debug, Serialize)]
pub struct GetGroupInfoRequest {
    pub group_id: i64,
    #[serde(default)]
    pub no_cache: bool,
}
impl Entry for GetGroupInfoRequest {
    const ENTRY: &'static str = "get_group_info";
    type Output = GroupInfo;
}
impl CqReq for GetGroupInfoRequest {}
impl JsonReq for GetGroupInfoRequest {}

#[derive(Debug, Serialize)]
pub struct GetGroupListRequest {}
impl Entry for GetGroupListRequest {
    const ENTRY: &'static str = "get_group_list";
    type Output = Vec<GroupInfo>;
}
impl CqReq for GetGroupListRequest {}
impl JsonReq for GetGroupListRequest {}

#[derive(Debug, Deserialize)]
pub struct GroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: Sex,
    pub age: i32,
    pub area: String,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: GroupRole,
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: i32,
    pub card_changeable: bool,
}

#[derive(Debug, Serialize)]
pub struct GetGroupMemberInfoRequest {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(default)]
    pub no_cache: bool,
}
impl Entry for GetGroupMemberInfoRequest {
    const ENTRY: &'static str = "get_group_member_info";
    type Output = GroupMemberInfo;
}
impl CqReq for GetGroupMemberInfoRequest {}
impl JsonReq for GetGroupMemberInfoRequest {}

#[derive(Debug, Serialize)]
pub struct GetGroupMemberListRequest {
    pub group_id: i64,
}
impl Entry for GetGroupMemberListRequest {
    const ENTRY: &'static str = "get_group_member_list";
    type Output = Vec<GroupMemberInfo>;
}
impl CqReq for GetGroupMemberListRequest {}
impl JsonReq for GetGroupMemberListRequest {}

#[derive(Debug, Deserialize)]
pub struct CurrentTalkative {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub day_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct HonorInfo {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
}

// TODO: cleanup
#[derive(Debug, Deserialize)]
pub struct GroupHonorInfo {
    pub group_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_talkative: Option<CurrentTalkative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talkative_list: Option<Vec<HonorInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer_list: Option<Vec<HonorInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_list: Option<Vec<HonorInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strong_newbie_list: Option<Vec<HonorInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion_list: Option<Vec<HonorInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HonorType {
    Talkative,
    Performer,
    Legend,
    StrongNewbie,
    Emotion,
    All,
}

#[derive(Debug, Serialize)]
pub struct GetGroupHonorInfoRequest {
    pub group_id: i64,
    pub r#type: HonorType,
}
impl Entry for GetGroupHonorInfoRequest {
    const ENTRY: &'static str = "get_group_honor_info";
    type Output = GroupHonorInfo;
}
impl CqReq for GetGroupHonorInfoRequest {}
impl JsonReq for GetGroupHonorInfoRequest {}

#[derive(Debug, Deserialize)]
pub struct CookiesResponse {
    pub cookies: String,
}

#[derive(Debug, Serialize)]
pub struct GetCookiesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}
impl Entry for GetCookiesRequest {
    const ENTRY: &'static str = "get_cookies";
    type Output = CookiesResponse;
}
impl CqReq for GetCookiesRequest {}
impl JsonReq for GetCookiesRequest {}

#[derive(Debug, Deserialize)]
pub struct CsrfTokenResponse {
    pub token: i32,
}

#[derive(Debug, Serialize)]
pub struct GetCsrfTokenRequest {}
impl Entry for GetCsrfTokenRequest {
    const ENTRY: &'static str = "get_csrf_token";
    type Output = CsrfTokenResponse;
}
impl CqReq for GetCsrfTokenRequest {}
impl JsonReq for GetCsrfTokenRequest {}

#[derive(Debug, Deserialize)]
pub struct CredentialsResponse {
    pub cookies: String,
    pub csrf_token: i32,
}

#[derive(Debug, Serialize)]
pub struct GetCredentialsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}
impl Entry for GetCredentialsRequest {
    const ENTRY: &'static str = "get_credentials";
    type Output = CredentialsResponse;
}
impl CqReq for GetCredentialsRequest {}
impl JsonReq for GetCredentialsRequest {}

#[derive(Debug, Deserialize)]
pub struct RecordResponse {
    pub file: String,
}

#[derive(Debug, Serialize)]
pub struct GetRecordRequest {
    pub file: String,
    pub out_format: String,
}
impl Entry for GetRecordRequest {
    const ENTRY: &'static str = "get_record";
    type Output = RecordResponse;
}
impl CqReq for GetRecordRequest {}
impl JsonReq for GetRecordRequest {}

#[derive(Debug, Deserialize)]
pub struct ImageResponse {
    pub file: String,
}

#[derive(Debug, Serialize)]
pub struct GetImageRequest {
    pub file: String,
}
impl Entry for GetImageRequest {
    const ENTRY: &'static str = "get_image";
    type Output = ImageResponse;
}
impl CqReq for GetImageRequest {}
impl JsonReq for GetImageRequest {}

#[derive(Debug, Deserialize)]
pub struct CanSendImageResponse {
    pub yes: bool,
}

#[derive(Debug, Serialize)]
pub struct CanSendImageRequest {}
impl Entry for CanSendImageRequest {
    const ENTRY: &'static str = "can_send_image";
    type Output = CanSendImageResponse;
}
impl CqReq for CanSendImageRequest {}
impl JsonReq for CanSendImageRequest {}

#[derive(Debug, Deserialize)]
pub struct CanSendRecordResponse {
    pub yes: bool,
}

#[derive(Debug, Serialize)]
pub struct CanSendRecordRequest {}
impl Entry for CanSendRecordRequest {
    const ENTRY: &'static str = "can_send_record";
    type Output = CanSendRecordResponse;
}
impl CqReq for CanSendRecordRequest {}
impl JsonReq for CanSendRecordRequest {}

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub online: Option<bool>,
    pub good: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetStatusRequest {}
impl Entry for GetStatusRequest {
    const ENTRY: &'static str = "get_status";
    type Output = StatusResponse;
}
impl CqReq for GetStatusRequest {}
impl JsonReq for GetStatusRequest {}

#[derive(Debug, Deserialize)]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
}

#[derive(Debug, Serialize)]
pub struct GetVersionInfoRequest {}
impl Entry for GetVersionInfoRequest {
    const ENTRY: &'static str = "get_version_info";
    type Output = VersionInfo;
}
impl CqReq for GetVersionInfoRequest {}
impl JsonReq for GetVersionInfoRequest {}

#[derive(Debug, Serialize)]
pub struct SetRestartRequest {
    #[serde(default)]
    pub delay: i32,
}
impl Entry for SetRestartRequest {
    const ENTRY: &'static str = "set_restart";
    type Output = Empty;
}
impl CqReq for SetRestartRequest {}
impl JsonReq for SetRestartRequest {}

#[derive(Debug, Serialize)]
pub struct CleanCacheRequest {}
impl Entry for CleanCacheRequest {
    const ENTRY: &'static str = "clean_cache";
    type Output = Empty;
}
impl CqReq for CleanCacheRequest {}
impl JsonReq for CleanCacheRequest {}
