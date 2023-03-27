use crate::airdrop::{Airdrop, BearerAirdrop, OpenAirdrop};
use crate::error::ContractError;
use crate::state::{airdrops, load_airdrop};
use cosmwasm_std::{attr, coins, Addr, Attribute, BankMsg, DepsMut, Env, MessageInfo, Response};
use ibcx_interface::airdrop::{AirdropId, AirdropType};

pub fn close(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: AirdropId,
) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;

    match airdrop {
        Airdrop::Open(inner) => close_open(deps, env, info, airdrop_id, inner),
        Airdrop::Bearer(inner) => close_bearer(deps, env, info, airdrop_id, inner),
    }
}

fn close_event(sender: Addr, typ: AirdropType, id: u64, redeem: impl Into<u128>) -> Vec<Attribute> {
    vec![
        attr("method", "close"),
        attr("executor", sender),
        attr("airdrop_type", typ),
        attr("airdrop_id", id.to_string()),
        attr("redeemed", redeem.into().to_string()),
    ]
}

fn close_open(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    airdrop_id: u64,
    mut airdrop: OpenAirdrop,
) -> Result<Response, ContractError> {
    // validation
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let redeem_amount = airdrop.total_amount.checked_sub(airdrop.total_claimed)?;

    airdrop.closed_at = Some(env.block.height);

    // response
    let send_msg = BankMsg::Send {
        to_address: airdrop.creator.to_string(),
        amount: coins(redeem_amount.u128(), &airdrop.denom),
    };

    let attrs = close_event(info.sender, AirdropType::Open, airdrop_id, redeem_amount);

    // apply states
    airdrops().save(deps.storage, airdrop_id, &airdrop.into())?;

    Ok(Response::new().add_message(send_msg).add_attributes(attrs))
}

fn close_bearer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    airdrop_id: u64,
    mut airdrop: BearerAirdrop,
) -> Result<Response, ContractError> {
    // validation
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let redeem_amount = airdrop.total_amount.checked_sub(airdrop.total_claimed)?;

    airdrop.closed_at = Some(env.block.height);

    // response
    let send_msg = BankMsg::Send {
        to_address: airdrop.creator.to_string(),
        amount: coins(redeem_amount.u128(), &airdrop.denom),
    };

    let attrs = close_event(info.sender, AirdropType::Bearer, airdrop_id, redeem_amount);

    // apply states
    airdrops().save(deps.storage, airdrop_id, &airdrop.into())?;

    Ok(Response::new().add_message(send_msg).add_attributes(attrs))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{
            mock_dependencies_with_balances, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        },
        Addr, OwnedDeps,
    };
    use ibcx_interface::airdrop::{AirdropId, AirdropType, InstantiateMsg};

    use crate::{
        contract::instantiate,
        execute::{
            close::close_event,
            tests::{
                mock_bearer_airdrop, mock_open_airdrop, normalize_amount, register_airdrop,
                Balances,
            },
        },
    };

    use super::close;

    fn setup(airdrop_type: AirdropType) -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let env = mock_env();

        let initial_balances: Balances = &[];

        let mut deps = mock_dependencies_with_balances(initial_balances);

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        match airdrop_type {
            AirdropType::Open => {
                let mock_airdrop = mock_open_airdrop(None, env.block.height);
                register_airdrop(deps.as_mut(), env, mock_airdrop.into(), None);
            }
            AirdropType::Bearer => {
                let (mock_airdrop, mock_airdrop_sign) = mock_bearer_airdrop(None, env.block.height);
                register_airdrop(
                    deps.as_mut(),
                    env,
                    mock_airdrop.into(),
                    Some(mock_airdrop_sign),
                );
            }
        }

        deps
    }

    #[test]
    fn test_close_open() {
        let mut deps = setup(AirdropType::Open);

        let close_resp = close(
            deps.as_mut(),
            mock_env(),
            mock_info("tester", &[]),
            AirdropId::id(0),
        )
        .unwrap();
        assert_eq!(
            close_resp.attributes,
            close_event(
                Addr::unchecked("tester"),
                AirdropType::Open,
                0,
                normalize_amount(0.01)
            )
        );
    }

    #[test]
    fn test_close_bearer() {
        let mut deps = setup(AirdropType::Bearer);

        let close_resp = close(
            deps.as_mut(),
            mock_env(),
            mock_info("tester", &[]),
            AirdropId::id(0),
        )
        .unwrap();
        assert_eq!(
            close_resp.attributes,
            close_event(
                Addr::unchecked("tester"),
                AirdropType::Bearer,
                0,
                normalize_amount(0.1)
            )
        );
    }
}
