use ibc_rs_scale::core::ics24_host::identifier::{ChannelId, PortId};

use crate::prelude::*;

use super::MemoData;

#[derive(
	Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct Forward {
	pub receiver: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub port: Option<PortId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel: Option<ChannelId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub retries: Option<u8>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub substrate: Option<IbcSubstrate>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub next: Option<Box<MemoData>>,
}

#[derive(
	Serialize,
	Deserialize,
	Clone,
	Debug,
	PartialEq,
	Eq,
	Encode,
	Decode,
	scale_info::TypeInfo,
	Default,
	Copy,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct IbcSubstrate {
	/// since other parachain does not support ibc memo
	/// there is only two option: send to parachain or send to relay-chain
	/// if para id is none, it means send to relay-chain
	#[serde(skip_serializing_if = "Option::is_none")]
	pub para_id: Option<u32>, //if para id is none, it means send to relay-chain
}

impl IbcSubstrate {
	pub fn new(para_id: Option<u32>) -> Self {
		Self { para_id }
	}
}

impl Forward {
	pub fn new_ibc_memo(
		receiver: String,
		port: PortId,
		channel: ChannelId,
		timeout: String,
		retries: u8,
	) -> Self {
		Self {
			receiver,
			port: Some(port),
			channel: Some(channel),
			timeout: Some(timeout),
			retries: Some(retries),
			substrate: <_>::default(),
			next: None,
		}
	}

	pub fn new_xcm_memo(receiver: String, substrate: IbcSubstrate) -> Self {
		Self {
			receiver,
			port: None,
			channel: None,
			timeout: None,
			retries: None,
			substrate: Some(substrate),
			next: None,
		}
	}
}
