use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::Addr;

use crate::state::Blog;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateBlog {
        title: String,
        content: String,
    },
    UpdateBlog {
        blog_id: u64,
        title: String,
        content: String,
    },
    DeleteBlog {
        blog_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetBlogs returns the blogs of the given address
    #[returns(GetBlogsResponse)]
    GetBlogs { addr: Addr },
    // GetBlog returns the blog of the given address and id
    #[returns(GetBlogResponse)]
    GetBlog { addr: Addr, id: u64 },
}

#[cw_serde]
pub struct GetBlogsResponse {
    pub blogs: Vec<Blog>,
}

#[cw_serde]
pub struct GetBlogResponse {
    pub blog: Blog,
}
