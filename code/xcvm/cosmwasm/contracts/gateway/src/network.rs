use crate::{events::make_event, prelude::*, state::xcvm::IBC_CHANNEL_NETWORK};
use cosmwasm_std::{Response, Storage};
use xc_core::{gateway::NetworkItem, NetworkId};

use crate::state::{self, NETWORK, NETWORK_TO_NETWORK};

use crate::error::Result;

pub fn load_this(storage: &dyn Storage) -> Result<NetworkItem> {
	let this = state::load(storage)?;
	let this = NETWORK.load(storage, this.id)?;
	Ok(this)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct OtherNetwork {
	pub network: NetworkItem,
	pub connection: OtherNetworkItem,
}

pub fn load_other(storage: &dyn Storage, _other: NetworkId) -> Result<OtherNetwork> {
	let this = state::load(storage)?;
	let other = NETWORK.load(storage, this.id)?;
	let connection = NETWORK_TO_NETWORK.load(storage, (this.id, other.id))?;
	Ok(OtherNetwork { network: other, connection })
}

pub(crate) fn force_network_to_network(
	_: crate::auth::Auth<crate::auth::policy::Admin>,
	deps: cosmwasm_std::DepsMut,
	msg: xc_core::gateway::ForceNetworkToNetworkMsg,
) -> std::result::Result<cosmwasm_std::Response, crate::error::ContractError> {
	NETWORK_TO_NETWORK.save(deps.storage, (msg.from, msg.to), &msg.other)?;
	if let Some(ibc) = msg.other.xcvm_channel {
		IBC_CHANNEL_NETWORK.save(deps.storage, ibc.id.to_string(), &msg.to)?;
	}
	Ok(Response::new()
		.add_event(make_event("network_to_network.forced").add_attribute("to", msg.to.to_string())))
}

pub(crate) fn force_network(
	_auth: crate::auth::Auth<crate::auth::policy::Admin>,
	deps: cosmwasm_std::DepsMut,
	msg: NetworkItem,
) -> std::result::Result<cosmwasm_std::Response, crate::error::ContractError> {
	NETWORK.save(deps.storage, msg.id, &msg)?;
	Ok(Response::new()
		.add_event(make_event("network.forced").add_attribute("id", msg.id.to_string())))
}
