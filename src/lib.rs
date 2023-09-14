mod from_ethers;
mod to_ethers;
use crate::from_ethers::{from_ethers_i256, from_ethers_u256, to_big_decimal};
use crate::to_ethers::{to_ethers_i256, to_ethers_u256};
use bigdecimal::BigDecimal;
use ethers::types::{I256, U256};

pub trait BigDecimalEthersExt
where
    Self: Sized,
{
    fn to_ethers_u256(&self, decimals: u8) -> Option<U256>;
    fn to_ethers_i256(&self, decimals: u8) -> Option<I256>;
    fn from_ethers_u256(bn: &U256, decimals: u8) -> Option<Self>;
    fn from_ethers_i256(bn: &I256, decimals: u8) -> Option<Self>;
    fn from_bn_string(bn: String, decimals: u8) -> Option<Self>;
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

    fn from_bn_string(bn: String, decimals: u8) -> Option<Self> {
        to_big_decimal(bn, decimals)
    }
}
