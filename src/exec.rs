use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::msgs::*;
use crate::{error::ContractError, types::msg::ExecuteMsg};

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        // Handle the different execute messages here
        // For example:
        // Transfer { recipient, amount } => transfer(deps, env, info, recipient, amount),
        // Other messages...
    }
}
