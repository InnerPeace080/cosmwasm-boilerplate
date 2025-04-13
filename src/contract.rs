use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::types::{msg::InstantiateMsg, query::ContractVersionResp};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:power-flow-bridge";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let resp = Response::new().add_attribute("method", "instantiate");

    Ok(resp)
}

pub fn get_contract_version(deps: Deps) -> StdResult<ContractVersionResp> {
    let version = cw2::get_contract_version(deps.storage)?;
    Ok(ContractVersionResp {
        contract: version.contract,
        version: version.version,
    })
}
