use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::types::query::QueryMsg;
use crate::{contract, msgs::*};

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        GetContractVersion {} => to_json_binary(&contract::get_contract_version(deps)?),
        // Handle the different query messages here
        // For example:
        // Balance { address } => to_json_binary(&query_balance(deps, address)?),
        // Other queries...
    }
}
