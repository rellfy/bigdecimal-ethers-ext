use bigdecimal::num_bigint::Sign;
use bigdecimal::BigDecimal;
use ethers::prelude::*;
use ethers::types::{I256, U256};
use log::error;
use std::ops::{Add, Mul};
use std::string::ToString;
use bigdecimal::num_traits::real::Real;
use crate::BigDecimalEthersExt;

static MAX_I256: Lazy<U256> = Lazy::new(|| U256::from_dec_str(&I256::MAX.to_string()).unwrap());
static MIN_I256_ABS: Lazy<U256> = Lazy::new(|| {
    U256::from_dec_str(&I256::MIN.to_string().replace('-', ""))
        .unwrap()
        .add(1)
});

pub(crate) fn to_ethers_u256(bd: &BigDecimal, decimals: u8) -> Option<U256> {
    let (integer, mut decimal) = split(bd);
    // Remove redundant zeroes.
    decimal = decimal.trim_end_matches('0').to_owned();
    if decimal.len() > decimals as usize {
        // Would overflow.
        return None;
    }
    // Append zeroes while decimal count is not matching.
    while decimal.len() < decimals as usize {
        decimal.push('0');
    }
    let combined = format!("{integer}{decimal}");
    let result = U256::from_dec_str(&combined);
    match result {
        Ok(value) => Some(value),
        Err(e) => {
            error!("{e}");
            None
        }
    }
}

pub(crate) fn to_ethers_i256(bd: &BigDecimal, decimals: u8) -> Option<I256> {
    let sign = bd.sign();
    let Some(u256) = bd.abs().to_ethers_u256(decimals) else {
        return None;
    };
    let would_overflow = sign == Sign::Plus && u256.gt(&MAX_I256)
        || sign == Sign::Minus && u256.gt(&MIN_I256_ABS);
    if would_overflow {
        return None;
    }

    let mut i256 = I256::from_raw(u256);
    if sign == Sign::Minus {
        i256 = i256.mul(-1);
    }
    Some(i256)
}

/// Splits the number into its integer and decimal parts.
fn split(bd: &BigDecimal) -> (String, String) {
    let as_string = bd.to_string();
    let Some((integer, decimal)) = as_string.split_once(".") else {
        // There is no decimal part.
        return (as_string, "".to_owned());
    };
    (integer.to_owned(), decimal.to_owned())
}

#[cfg(test)]
mod test {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use crate::BigDecimalEthersExt;

    #[test]
    fn should_convert_big_decimal_u8_to_ethers_u256_with_18_decimals() {
        let value = BigDecimal::from_u8(10).unwrap();
        let u256 = value.to_ethers_u256(18).unwrap();
        assert_eq!(u256.to_string(), "10000000000000000000");
    }

    #[test]
    fn should_convert_big_decimal_f64_to_ethers_u256_with_18_decimals() {
        let value = BigDecimal::from_f64(12.5).unwrap();
        let u256 = value.to_ethers_u256(18).unwrap();
        assert_eq!(u256.to_string(), "12500000000000000000");
    }

    #[test]
    fn should_convert_big_decimal_f64_to_ethers_i256_with_18_decimals() {
        let value = BigDecimal::from_f64(-12.5).unwrap();
        let i256 = value.to_ethers_i256(18).unwrap();
        assert_eq!(i256.to_string(), "-12500000000000000000");
    }

    #[test]
    fn should_convert_big_decimal_f64_to_ethers_u256_with_2_decimals() {
        let value = BigDecimal::from_f64(12.5).unwrap();
        let u256 = value.to_ethers_u256(2).unwrap();
        assert_eq!(u256.to_string(), "1250");
    }

    #[test]
    fn should_convert_big_decimal_f64_to_ethers_i256_with_2_decimals() {
        let value = BigDecimal::from_f64(-12.5).unwrap();
        let i256 = value.to_ethers_i256(2).unwrap();
        assert_eq!(i256.to_string(), "-1250");
    }

    #[test]
    fn should_convert_big_decimal_f64_to_ethers_u256_with_0_decimals() {
        let value = BigDecimal::from_f64(12.0).unwrap();
        let u256 = value.to_ethers_u256(0).unwrap();
        assert_eq!(u256.to_string(), "12");
    }

    #[test]
    fn should_convert_big_decimal_f64_to_ethers_i256_with_0_decimals() {
        let value = BigDecimal::from_f64(12.0).unwrap();
        let i256 = value.to_ethers_i256(0).unwrap();
        assert_eq!(i256.to_string(), "12");
    }

    #[test]
    fn should_not_convert_big_decimal_f64_to_ethers_u256_with_0_decimals() {
        // This should fail because data would be lost otherwise.
        let value = BigDecimal::from_f64(12.5).unwrap();
        let option = value.to_ethers_u256(0);
        assert!(option.is_none());
    }

    #[test]
    fn should_not_convert_big_decimal_f64_to_ethers_i256_with_0_decimals() {
        // This should fail because data would be lost otherwise.
        let value = BigDecimal::from_f64(12.5).unwrap();
        let option = value.to_ethers_i256(0);
        assert!(option.is_none());
    }
}
