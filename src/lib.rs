mod from_ethers;
mod to_ethers;
use bigdecimal::BigDecimal;
use ethers::types::{I256, U256};
pub use from_ethers::*;
pub use to_ethers::*;

pub trait BigDecimalEthersExt
where
    Self: Sized,
{
    fn to_ethers_u256(&self, decimals: u8) -> Option<U256>;
    fn to_ethers_i256(&self, decimals: u8) -> Option<I256>;
    fn from_ethers_u256(bn: &U256, decimals: u8) -> Option<Self>;
    fn from_ethers_i256(bn: &I256, decimals: u8) -> Option<Self>;
}

impl BigDecimalEthersExt for BigDecimal {
    fn to_ethers_u256(&self, decimals: u8) -> Option<U256> {
        to_ethers_u256(self, decimals)
    }

    fn to_ethers_i256(&self, decimals: u8) -> Option<I256> {
        to_ethers_i256(self, decimals)
    }

    fn from_ethers_u256(bn: &U256, decimals: u8) -> Option<Self> {
        from_ethers_u256(bn, decimals)
    }

    fn from_ethers_i256(bn: &I256, decimals: u8) -> Option<Self> {
        from_ethers_i256(bn, decimals)
    }
}
