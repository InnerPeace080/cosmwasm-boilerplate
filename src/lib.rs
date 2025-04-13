#[cfg(feature = "library")]
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
#[cfg(feature = "library")]
use error::ContractError;
#[cfg(feature = "library")]
use types::{
    msg::{ExecuteMsg, InstantiateMsg},
    query::QueryMsg,
    sudo::SudoMsg,
};

pub mod constant;
#[cfg(feature = "library")]
pub mod contract;
#[cfg(feature = "types")]
pub mod error;
#[cfg(feature = "library")]
pub mod exec;
#[cfg(feature = "library")]
pub mod msgs;
#[cfg(feature = "library")]
pub mod query;
#[cfg(feature = "library")]
pub mod state;
#[cfg(feature = "types")]
pub mod types;

#[cfg(test)]
pub mod test;

#[cfg(feature = "library")]
#[cfg_attr(feature = "contract", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[cfg(feature = "library")]
#[cfg_attr(feature = "contract", entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    query::query(deps, env, msg)
}

#[cfg(feature = "library")]
#[cfg_attr(feature = "contract", entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    exec::execute(deps, env, info, msg)
}

#[cfg(feature = "library")]
#[cfg_attr(feature = "contract", entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {}
}

#[cfg(feature = "library")]
#[cfg_attr(feature = "contract", entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    use cosmwasm_std::StdError;

    match msg.id {
        _ => Err(ContractError::Std(StdError::generic_err(format!(
            "Invalid sub msg id {:?}",
            msg.id
        )))),
    }
}
