use crate::{
	auth,
	error::{ContractError, Result},
	events::make_event,
	msg, state,
};
use cosmwasm_std::{Deps, DepsMut, Response};
use xc_core::AssetId;

/// Adds a new asset to the registry; errors out if asset already exists.
pub(crate) fn handle_register_asset(
	_: auth::Admin,
	deps: DepsMut,
	asset_id: AssetId,
	reference: msg::Asset,
) -> Result {
	let key = state::ASSETS.key(asset_id);
	if key.has(deps.storage) {
		return Err(ContractError::AlreadyRegistered)
	}
	key.save(deps.storage, &reference)?;
	Ok(Response::new().add_event(
		make_event("register")
			.add_attribute("asset_id", asset_id.to_string())
			.add_attribute("denom", reference.denom()),
	))
}

/// Removes an existing asset from the registry; errors out if asset doesn’t
/// exist.
pub(crate) fn handle_unregister_asset(_: auth::Admin, deps: DepsMut, asset_id: AssetId) -> Result {
	let key = state::ASSETS.key(asset_id);
	if !key.has(deps.storage) {
		return Err(ContractError::UnsupportedAsset)
	}
	key.remove(deps.storage);
	Ok(Response::new()
		.add_event(make_event("unregister").add_attribute("asset_id", asset_id.to_string())))
}

/// Fetches information about given asset.
pub(crate) fn query_lookup(deps: Deps, asset_id: AssetId) -> Result<msg::LookupResponse> {
	state::ASSETS
		.may_load(deps.storage, asset_id)?
		.map(|reference| msg::LookupResponse { reference })
		.ok_or(ContractError::UnsupportedAsset)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		contract::{execute, query},
		msg, state,
	};
	use cosmwasm_std::{
		from_binary,
		testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
		Addr, Empty, Env, MessageInfo, Order, OwnedDeps, Response,
	};
	use xc_core::{
		gateway::{Asset, RegisterAssetMsg},
		Network,
	};

	fn instantiate(
	) -> (OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>, Env, MessageInfo, Response) {
		let sender = "sender";
		let msg = msg::InstantiateMsg {
			interpreter_code_id: 0,
			network_id: 1.into(),
			admin: sender.into(),
			ibc_ics_20_sender: None,
		};
		let mut deps = mock_dependencies();
		let env = mock_env();
		let info = mock_info(sender, &vec![]);
		let resp =
			crate::contract::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
		(deps, env, info, resp)
	}

	#[test]
	fn proper_instantiation() {
		let (_deps, _env, _info, resp) = instantiate();
		assert_eq!(0, resp.messages.len());
	}

	#[test]
	fn register_unregister_assets() {
		let (mut deps, env, info, _resp) = instantiate();

		let addr1 = Asset {
			network_id: xc_core::Picasso::ID,
			local: msg::AssetReference::Virtual { cw20_address: Addr::unchecked("addr1") },
			bridged: None,
		};
		let addr2 = Asset {
			network_id: xc_core::Picasso::ID,
			local: msg::AssetReference::Virtual { cw20_address: Addr::unchecked("addr2") },
			bridged: None,
		};
		let addr3 = Asset {
			network_id: xc_core::Picasso::ID,
			local: msg::AssetReference::Virtual { cw20_address: Addr::unchecked("addr3") },
			bridged: None,
		};
		let addr4 = Asset {
			network_id: xc_core::Picasso::ID,
			local: msg::AssetReference::Virtual { cw20_address: Addr::unchecked("addr4") },
			bridged: None,
		};

		execute::execute(
			deps.as_mut(),
			env.clone(),
			info.clone(),
			msg::ExecuteMsg::RegisterAsset(RegisterAssetMsg { id: 1.into(), asset: addr1.clone() }),
		)
		.unwrap();

		execute::execute(
			deps.as_mut(),
			env.clone(),
			info.clone(),
			msg::ExecuteMsg::RegisterAsset(RegisterAssetMsg { id: 2.into(), asset: addr2.clone() }),
		)
		.unwrap();

		assert_eq!(state::ASSETS.load(&deps.storage, 1.into()).unwrap(), addr1);
		assert_eq!(state::ASSETS.load(&deps.storage, 2.into()).unwrap(), addr2);

		execute::execute(
			deps.as_mut(),
			env.clone(),
			info.clone(),
			msg::ExecuteMsg::UnregisterAsset { asset_id: 1.into() },
		)
		.unwrap();

		execute::execute(
			deps.as_mut(),
			env.clone(),
			info.clone(),
			msg::ExecuteMsg::UnregisterAsset { asset_id: 2.into() },
		)
		.unwrap();

		execute::execute(
			deps.as_mut(),
			env.clone(),
			info.clone(),
			msg::ExecuteMsg::RegisterAsset(RegisterAssetMsg { id: 3.into(), asset: addr3.clone() }),
		)
		.unwrap();

		execute::execute(
			deps.as_mut(),
			env.clone(),
			info.clone(),
			msg::ExecuteMsg::RegisterAsset(RegisterAssetMsg { id: 4.into(), asset: addr4.clone() }),
		)
		.unwrap();

		// Make sure that set removes the previous elements
		assert!(state::ASSETS.load(&deps.storage, 1.into()).is_err());
		assert!(state::ASSETS.load(&deps.storage, 2.into()).is_err());
		assert_eq!(state::ASSETS.load(&deps.storage, 3.into()).unwrap(), addr3);
		assert_eq!(state::ASSETS.load(&deps.storage, 4.into()).unwrap(), addr4);

		// Finally make sure that there are two elements in the assets storage
		assert_eq!(
			state::ASSETS
				.keys(&deps.storage, None, None, Order::Ascending)
				.collect::<Vec<_>>()
				.len(),
			2
		);
	}

	#[test]
	fn query_assets() {
		let (mut deps, env, info, _resp) = instantiate();

		let addr1 = Asset {
			network_id: xc_core::Picasso::ID,
			local: msg::AssetReference::Virtual { cw20_address: Addr::unchecked("addr1") },
			bridged: None,
		};

		let asset_id = AssetId::from(1);
		execute::execute(
			deps.as_mut(),
			env.clone(),
			info.clone(),
			msg::ExecuteMsg::RegisterAsset(RegisterAssetMsg { id: asset_id, asset: addr1.clone() }),
		)
		.unwrap();

		let res: msg::LookupResponse = from_binary(
			&query(deps.as_ref(), env.clone(), msg::QueryMsg::LookupAsset { asset_id }).unwrap(),
		)
		.unwrap();

		// Query should return the corresponding address
		assert_eq!(res, msg::LookupResponse { reference: addr1 });

		// This should fail since there the asset doesn't exist
		assert!(query(
			deps.as_ref(),
			env.clone(),
			msg::QueryMsg::LookupAsset { asset_id: AssetId::from(2) }
		)
		.is_err());
	}
}
