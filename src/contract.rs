#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::BLOGS;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:blog";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateBlog { title, content } => {
            execute::create_blog(deps, env, info, title, content)
        }
        ExecuteMsg::UpdateBlog {
            blog_id,
            title,
            content,
        } => execute::update_blog(deps, env, info, blog_id, title, content),
        ExecuteMsg::DeleteBlog { blog_id } => execute::delete_blog(deps, info, blog_id),
    }
}

pub mod execute {

    use crate::state::Blog;

    use super::*;

    pub fn create_blog(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        title: String,
        content: String,
    ) -> Result<Response, ContractError> {
        let mut blogs = BLOGS.load(deps.storage, info.sender.clone())?;

        let blog = Blog {
            id: (blogs.len() + 1) as u64,
            title,
            content,
            created_at: env.block.time.seconds(),
            updated_at: env.block.time.seconds(),
        };

        blogs.push(blog);
        BLOGS.save(deps.storage, info.sender.clone(), &blogs)?;
        Ok(Response::new())
    }

    pub fn update_blog(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        blog_id: u64,
        title: String,
        content: String,
    ) -> Result<Response, ContractError> {
        let mut blogs = BLOGS.load(deps.storage, info.sender.clone())?;
        let blog = blogs
            .get_mut(blog_id as usize)
            .ok_or(ContractError::NotFound {})?;
        blog.title = title;
        blog.content = content;
        blog.updated_at = env.block.time.seconds();
        BLOGS.save(deps.storage, info.sender.clone(), &blogs)?;
        Ok(Response::new())
    }

    pub fn delete_blog(
        deps: DepsMut,
        info: MessageInfo,
        blog_id: u64,
    ) -> Result<Response, ContractError> {
        let mut blogs = BLOGS.load(deps.storage, info.sender.clone())?;
        blogs.remove(blog_id as usize);
        BLOGS.save(deps.storage, info.sender.clone(), &blogs)?;
        Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBlogs { addr } => to_json_binary(&query::get_blogs(deps, addr)?),
        QueryMsg::GetBlog { addr, id } => to_json_binary(&query::get_blog(deps, addr, id)?),
    }
}

pub mod query {
    use cosmwasm_std::StdError;

    use crate::msg::{GetBlogResponse, GetBlogsResponse};

    use super::*;
    pub fn get_blogs(deps: Deps, addr: Addr) -> StdResult<GetBlogsResponse> {
        let blogs = BLOGS.load(deps.storage, addr)?;
        Ok(GetBlogsResponse { blogs })
    }

    pub fn get_blog(deps: Deps, addr: Addr, id: u64) -> StdResult<GetBlogResponse> {
        let blogs = BLOGS.load(deps.storage, addr)?;
        let blog = blogs
            .get(id as usize)
            .cloned()
            .ok_or(StdError::generic_err("Blog not found"))?;
        Ok(GetBlogResponse { blog })
    }
}

#[cfg(test)]
mod tests {}
