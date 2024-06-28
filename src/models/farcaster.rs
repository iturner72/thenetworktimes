use serde::{Deserialize, Serialize};

// casts

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cast {
    pub data: CastData,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CastData {
    pub cast_add_body: Option<CastAddBody>,
    pub fid: u64,
    pub network: String,
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub cast_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CastAddBody {
    pub embeds: Vec<Embed>,
    pub embeds_deprecated: Vec<String>,
    pub mentions: Vec<u64>,
    pub mentions_positions: Vec<u32>,
    pub parent_cast_id: Option<ParentCastId>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Embed {
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParentCastId  {
    pub fid: u64,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CastResponse {
    pub messages: Vec<Cast>,
}

// user

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDataResponse {
    pub data: UserData,
    pub hash: String,
    #[serde(default)]
    pub hash_scheme: Option<String>,
    pub signature: String,
    #[serde(default)]
    pub signature_scheme: Option<String>,
    pub signer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub fid: u64,
    pub network: String,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "userDataBody")]
    pub user_data_body: UserDataBody,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDataBody {
    #[serde(rename = "type")]
    pub data_type: String,
    pub value: String,
}

// reactions

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionResponse {
    pub fid: u64,
    pub hash: String,
    pub author_fid: u64,
    pub target_hash: String,
    pub timestamp: u64,
    pub reaction_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionData {
    pub fid: u64,
    pub network: String,
    pub reaction_body: ReactionBody,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub reaction_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionBody {
    pub target_cast_id: TargetCastId,
    #[serde(rename = "type")]
    pub reaction_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetCastId {
    pub fid: u64,
    pub hash: String,
}

// channels

#[derive(Clone, Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Channel {
    pub createdAt: u64,
    pub description: String,
    pub followerCount: u64,
    pub id: String,
    pub imageUrl: String,
    pub leadFid: u64,
    pub moderatorFid: Option<u64>,
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChannelsResult {
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChannelsResponse {
    pub result: ChannelsResult,
}
