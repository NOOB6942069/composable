use codec::FullCodec;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Zero},
	ArithmeticError,
};
use sp_std::fmt::Debug;

use composable_support::math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub};

/// really u8, but easy to do math operations
pub type Exponent = u32;

/// Creates a new asset, compatible with [`MultiCurrency`](https://docs.rs/orml-traits/0.4.0/orml_traits/currency/trait.MultiCurrency.html).
/// The implementor should ensure that a new `CurrencyId` is created and collisions are avoided.
/// Is about Local assets representations. These may differ remotely.
pub trait CurrencyFactory<AssetId, Balance> {
	/// permissionless creation of new transferable asset id
	fn create(id: RangeId, ed: Balance) -> Result<AssetId, DispatchError>;
	fn reserve_lp_token_id(ed: Balance) -> Result<AssetId, DispatchError> {
		Self::create(RangeId::LP_TOKENS, ed)
	}
	/// Given a local `AssetId` (within https://github.com/ComposableFi/composable/pull/1651n the range of `0` to `u32::MAX`) returns a global (within the
	/// scope of Currency Factory) `AssetId`.
	fn local_to_global_asset_id(
		asset_id: AssetId,
		range_id: RangeId,
	) -> Result<AssetId, DispatchError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct RangeId(u32);

impl RangeId {
	pub const LP_TOKENS: RangeId = RangeId(0);
	pub const TOKENS: RangeId = RangeId(1);
	pub const FOREIGN_ASSETS: RangeId = RangeId(2);
	pub const IBC_ASSETS: RangeId = RangeId(3);
	pub const FNFT_ASSETS: RangeId = RangeId(4);
	pub const XTOKEN_ASSETS: RangeId = RangeId(5);

	pub fn inner(&self) -> u32 {
		self.0
	}
}

impl From<u32> for RangeId {
	fn from(i: u32) -> Self {
		RangeId(i)
	}
}

/// Local presentation of asset information.
/// Most pallets do not need it.
pub trait LocalAssets<MayBeAssetId> {
	/// decimals of of big unit over minimal unit.
	/// orml also has separate trait on Balances to inspect decimals, that is not on type it self
	fn decimals(currency_id: MayBeAssetId) -> Result<Exponent, DispatchError>;

	/// Amount which humans operate as `1` usually.
	/// Amount is probably priceable by Oracles.
	/// Amount reasonably higher than minimal tradeable amount or minimal trading step on DEX.
	fn unit<T: From<u64>>(currency_id: MayBeAssetId) -> Result<T, DispatchError> {
		let exponent = Self::decimals(currency_id)?;
		Ok(10_u64.checked_pow(exponent).ok_or(ArithmeticError::Overflow)?.into())
	}
}

/// when we store assets in native form to chain in smallest units or for mock in tests
impl<MayBeAssetId> LocalAssets<MayBeAssetId> for () {
	fn decimals(_currency_id: MayBeAssetId) -> Result<Exponent, DispatchError> {
		Ok(12)
	}
}

// FIXME(hussein-aitlahcen): this trait already exist under frame_support, named Balance
pub trait BalanceLike:
	AtLeast32BitUnsigned
	+ FullCodec
	+ Copy
	+ Default
	+ Debug
	+ MaybeSerializeDeserialize
	+ MaxEncodedLen
	+ TypeInfo
{
}
impl<T> BalanceLike for T where
	T: AtLeast32BitUnsigned
		+ FullCodec
		+ Copy
		+ Default
		+ Debug
		+ MaybeSerializeDeserialize
		+ MaxEncodedLen
		+ TypeInfo
{
}

/// limited counted number trait which maximal number is more than `u64`,  but not more than `u128`,
/// so inner type is either u64 or u128 with helpers for producing `ArithmeticError`s instead of
/// `Option`s.
pub trait MathBalance:
	PartialOrd
	+ Zero
	+ SafeAdd
	+ SafeDiv
	+ SafeMul
	+ SafeSub
	+ Into<u128>
	+ TryFrom<u128>
	+ From<u64>
	+ Copy
{
}
impl<
		T: PartialOrd
			+ Zero
			+ SafeAdd
			+ SafeDiv
			+ SafeMul
			+ SafeSub
			+ Into<u128>
			+ TryFrom<u128>
			+ From<u64>
			+ Copy,
	> MathBalance for T
{
}

// hack to imitate type alias until it is in stable
// named with like implying it is`like` is is necessary to be `AssetId`, but may be not enough (if
// something is `AssetIdLike` than it is not always asset)

// FIXME(hussein-aitlahcen): this trait already exists in frame_support, named `AssetId`
pub trait AssetIdLike:
	FullCodec + MaxEncodedLen + Copy + Eq + PartialEq + Debug + TypeInfo
{
}
impl<T: FullCodec + MaxEncodedLen + Copy + Eq + PartialEq + Debug + TypeInfo> AssetIdLike for T {}
