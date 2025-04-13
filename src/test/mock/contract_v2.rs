use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::{ensure_from_older_version, set_contract_version};

use crate::{
    error::ContractError,
    state,
    types::msg::{InstantiateMsg, MigrateMsg},
};

pub const CONTRACT_NAME: &str = "crates.io:power-flow-bridge";
pub const CONTRACT_VERSION: &str = "10.0.0-rc.1";

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Initialize the contract state
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "migrate"))
}
