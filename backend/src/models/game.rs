use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Game {
    pub id: u64,
    pub players: HashMap<i32, Player>,
    #[serde(rename(deserialize = "boardType"))]
    pub board_type: String,
    pub seed: u64,
    pub log: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Player {
    #[serde(rename(deserialize = "webId"))]
    pub web_id: u64,
    pub elo: f32,
    pub karma: u32,
    pub corporation: String,
}
