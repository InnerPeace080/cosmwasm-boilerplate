use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get the contract version
    #[returns(ContractVersionResp)]
    #[serde(rename = "get_contract_version")]
    GetContractVersion {},
}

#[cw_serde]
pub struct ContractVersionResp {
    /// The contract version
    pub contract: String,

    pub version: String,
}
