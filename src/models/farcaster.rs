use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Cast {
        pub fid: u64,
        pub hash: String,
        pub parent_hash: Option<String>,
        pub author_fid: u64,
        pub timestamp: u64,
        pub text: String,
        pub mentions: Vec<u64>,
        pub mentions_positions: Vec<u32>,
        pub embeds: Vec<Embed>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub fid: u64,
        pub username: String,
        pub display_name: String,
        pub bio: String,
        pub avatar_url: String,
        pub verified: bool,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Reaction {
        pub fid: u64,
        pub hash: String,
        pub author_fid: u64,
        pub target_hash: String,
        pub timestamp: u64,
        pub reaction_type: String,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Embed {
        pub url: Option<String>,
        pub cast_id: Option<CastId>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CastId {
        pub fid: u64,
        pub hash: String,
    }
}}
