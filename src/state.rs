use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Blog {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at: u64,
    pub updated_at: u64,
}

pub const BLOGS: Map<Addr, Vec<Blog>> = Map::new("blogs");
