use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
    pub x_factor: i32,
    pub members_list: Option<Vec<Addr>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    IncrementXFactor {},
    AddMeToWaitingList {},
    AddWaitingListToClub {},
    AddMemberToClub { prospect: Addr},
    Reset { count: i32 },
    ResetXFactor { x_factor: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    GetXFactor {},
    GetMemberList {},
    GetWaitingList {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct XFactorResponse {
    pub x_factor: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct MemberListResponse {
    pub members_list: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct WaitingListResponse {
    pub waiting_list: Vec<Addr>,
}


