use cw_storage_plus::{Item, Map};
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

pub const BLOGS: Map<(Addr, u64), Blog> = Map::new("blogs");
pub const LATEST_BLOG_ID: Item<u64> = Item::new("latest_blog_id");
