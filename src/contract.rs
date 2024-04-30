use std::vec;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Addr,
};

use crate::msg::{CountResponse, XFactorResponse, ExecuteMsg, InstantiateMsg, QueryMsg, MemberListResponse, WaitingListResponse, MembersOnlyCountResponse};
use crate::state::{config, config_read, State};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let valid_members_list = match msg.members_list {
        Some(member_list) => {
            let validated_addrs: Result<Vec<_>, _> = member_list
                .iter()
                .map(|addr| deps.api.addr_validate(addr.as_str()))
                .collect();

            validated_addrs?
        },
        None => vec![info.sender.clone()]
    };
    let state = State {
        count: msg.count,
        x_factor: msg.x_factor,
        members_only_count: 0,
        owner: info.sender.clone(),
        members_list: valid_members_list,
        waiting_list: vec![],
    };

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps, env),
        ExecuteMsg::IncrementXFactor {} => try_increment_x_factor(deps, info),
        ExecuteMsg::IncrementMembersOnlyCount {} => try_increment_members_only_count(deps, info),
        ExecuteMsg::AddMeToWaitingList {} => try_add_me_to_waiting_list(deps, info),
        ExecuteMsg::AddMemberToClub { prospect } => try_add_member_to_club(deps, info, prospect),
        ExecuteMsg::AddWaitingListToClub {} => try_add_waiting_list_to_club(deps, info),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::ResetXFactor { x_factor } => try_reset_x_factor(deps, info, x_factor),
        ExecuteMsg::ResetMembersOnlyCount {} => try_reset_members_only_count(deps, info),
    }
}

pub fn try_increment(deps: DepsMut, _env: Env) -> StdResult<Response> {
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        state.count += 1;
        Ok(state)
    })?;

    deps.api.debug("count incremented successfully");
    Ok(Response::default())
}


pub fn try_increment_x_factor(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        if sender_address.to_string().contains("x") {
            state.x_factor += 1;
            Ok(state)
        } else {
            return Err(StdError::generic_err("You need an x in your address to increment count"));
        }
    })?;

    deps.api.debug("count incremented successfully");
    Ok(Response::default())

}

pub fn try_increment_members_only_count(deps: DepsMut,info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        if state.members_list.contains(&sender_address) {
            state.members_only_count += 1;
            Ok(state)
        } else {
            return Err(StdError::generic_err("You need to be on the members list to increment count"));
        }
    })?;

    deps.api.debug("Members only count incremented successfully");
    Ok(Response::default())
}

pub fn try_add_me_to_waiting_list(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        if (state.members_list.contains(&sender_address)) || (state.waiting_list.contains(&sender_address)) {
            return Err(StdError::generic_err("You are already part of the waiting list or members list!"));
        }
        state.waiting_list.push(sender_address);
        Ok(state)
        })?;

    deps.api.debug("You have been added to the waiting list!");
    Ok(Response::default())
}

pub fn try_add_member_to_club(deps: DepsMut, info: MessageInfo, prospect: Addr) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if state.members_list.contains(&sender_address) {
            state.members_list.push(prospect);
            Ok(state)
        } else {
            return Err(StdError::generic_err("Only club members can add a new member!"));
        }
    })?;

    deps.api.debug("Member added to club");
    Ok(Response::default())

}

pub fn try_add_waiting_list_to_club(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if state.members_list.contains(&sender_address) {
            let list_to_add = state.waiting_list;
            for prospect in list_to_add {

                state.members_list.push(prospect);
            }
            state.waiting_list = vec![];
            Ok(state)
        } else {
            return Err(StdError::generic_err("Only club members can do this action!"))
        }
    })?;

    deps.api.debug("Waiting list added to club.");
    Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if sender_address != state.owner || state.members_list.contains(&sender_address)  {
            return Err(StdError::generic_err("Only the owner can reset count"));
        }
        state.count = count;
        Ok(state)
    })?;

    deps.api.debug("count reset successfully");
    Ok(Response::default())
}

pub fn try_reset_x_factor(deps: DepsMut, info: MessageInfo, x_factor: i32) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        if sender_address.to_string().contains("x") || state.members_list.contains(&sender_address)  {
            state.x_factor = x_factor;
            Ok(state)
        } else {
            return Err(StdError::generic_err("You need an x in your address to reset count"));
        }
    })?;

    deps.api.debug("X factor reset successfully");
    Ok(Response::default())

}


pub fn try_reset_members_only_count(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| -> Result<_, StdError> {
        if state.members_list.contains(&sender_address) {
            state.members_only_count = 0;
            Ok(state)
        } else {
            return Err(StdError::generic_err("You need to be on the members list to reset this count"));
        }
    })?;

    deps.api.debug("Members only count reset successfully");
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetXFactor {} => to_binary(&query_x_factor(deps)?),
        QueryMsg::GetMembersOnlyCount {} => to_binary(&query_members_only_count(deps)?),
        QueryMsg::GetWaitingList {} => to_binary(&query_waiting_list(deps)?),
        QueryMsg::GetMemberList {} => to_binary(&query_members_list(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(CountResponse { count: state.count })
}

fn query_x_factor(deps: Deps) -> StdResult<XFactorResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(XFactorResponse { x_factor: state.x_factor })
}

fn query_members_only_count(deps: Deps) -> StdResult<MembersOnlyCountResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(MembersOnlyCountResponse { members_only_count: state.members_only_count })
}

fn query_members_list(deps: Deps) -> StdResult<MemberListResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(MemberListResponse { members_list: state.members_list })
}

fn query_waiting_list(deps: Deps) -> StdResult<WaitingListResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(WaitingListResponse { waiting_list: state.waiting_list })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );
        let init_msg = InstantiateMsg {
            count: 17,
            x_factor: 17,
        };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg {
            count: 17,
            x_factor: 17,
        };

        let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        // anyone can increment
        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );

        let exec_msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg {
            count: 17,
            x_factor: 17
        };

        let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        // not anyone can reset
        let info = mock_info(
            "anyone",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let exec_msg = ExecuteMsg::Reset { count: 5 };

        let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

        match res {
            Err(StdError::GenericErr { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let exec_msg = ExecuteMsg::Reset { count: 5 };

        let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
