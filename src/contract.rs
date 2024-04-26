use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::msg::{CountResponse, XFactorResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, State};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        count: msg.count,
        x_factor: msg.x_factor,
        owner: info.sender.clone(),
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
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::ResetXFactor { x_factor } => try_reset_x_factor(deps, info, x_factor),
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

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|mut state| {
        if sender_address != state.owner {
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
        if sender_address.to_string().contains("x") {
            state.x_factor = x_factor;
            Ok(state)
        } else {
            return Err(StdError::generic_err("You need an x in your address to reset count"));
        }
    })?;

    deps.api.debug("X factor reset successfully");
    Ok(Response::default())

}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetXFactor {} => to_binary(&query_x_factor(deps)?),
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
