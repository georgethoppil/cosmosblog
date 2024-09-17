#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BLOGS, LATEST_BLOG_ID};

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
    LATEST_BLOG_ID.save(deps.storage, &0)?;
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
        let id = LATEST_BLOG_ID.update(deps.storage, |id| -> StdResult<_> { Ok(id + 1) })?;

        let blog = Blog {
            id,
            title,
            content,
            created_at: env.block.time.seconds(),
            updated_at: env.block.time.seconds(),
        };

        BLOGS.save(deps.storage, (info.sender, id), &blog)?;
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
        BLOGS.update(
            deps.storage,
            (info.sender, blog_id),
            |blog| -> Result<_, ContractError> {
                match blog {
                    Some(mut blog) => {
                        blog.title = title;
                        blog.content = content;
                        blog.updated_at = env.block.time.seconds();
                        Ok(blog)
                    }
                    None => Err(ContractError::NotFound {}),
                }
            },
        )?;

        Ok(Response::new())
    }

    pub fn delete_blog(
        deps: DepsMut,
        info: MessageInfo,
        blog_id: u64,
    ) -> Result<Response, ContractError> {
        BLOGS.remove(deps.storage, (info.sender.clone(), blog_id));
        Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBlog { addr, id } => to_json_binary(&query::get_blog(deps, addr, id)?),
    }
}

pub mod query {

    use crate::msg::GetBlogResponse;

    use super::*;
    pub fn get_blog(deps: Deps, addr: Addr, id: u64) -> StdResult<GetBlogResponse> {
        let blog = BLOGS.load(deps.storage, (addr, id))?;
        Ok(GetBlogResponse { blog })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_create_blog() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = ExecuteMsg::CreateBlog {
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {}).unwrap();

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let rsp = query::get_blog(deps.as_ref(), info.sender.clone(), 1).unwrap();
        assert_eq!(rsp.blog.title, "Test Title");
        assert_eq!(rsp.blog.content, "Test Content");
    }

    #[test]
    fn test_update_blog() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = ExecuteMsg::CreateBlog {
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {}).unwrap();
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::UpdateBlog {
            blog_id: 1,
            title: "Updated Title".to_string(),
            content: "Updated Content".to_string(),
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let rsp = query::get_blog(deps.as_ref(), info.sender.clone(), 1).unwrap();
        assert_eq!(rsp.blog.title, "Updated Title");
        assert_eq!(rsp.blog.content, "Updated Content");
    }

    #[test]
    fn test_delete_blog() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = ExecuteMsg::CreateBlog {
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
        };
        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {}).unwrap();
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::DeleteBlog { blog_id: 1 };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let rsp = query::get_blog(deps.as_ref(), info.sender.clone(), 1);
        assert_eq!(rsp.is_err(), true);
    }
}
