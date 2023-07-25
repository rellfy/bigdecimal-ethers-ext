use crate::BigDecimalEthersExt;
use bigdecimal::{BigDecimal, Num};
use ethers::prelude::{I256, U256};

pub(crate) fn from_ethers_u256(bn: U256, decimals: u8) -> Option<BigDecimal> {
    to_big_decimal(bn.to_string(), decimals)
}

pub(crate) fn from_ethers_i256(bn: I256, decimals: u8) -> Option<BigDecimal> {
    to_big_decimal(bn.to_string(), decimals)
}

fn to_big_decimal(bn_string: String, decimals: u8) -> Option<BigDecimal> {
    let decimals = decimals as usize;
    let Some(mut value_le) = reversed_string(bn_string) else {
        return None;
    };
    // e.g. for bn_str = "0", decimals = 1; must pad to "00" and return "0.0".
    right_pad(&mut value_le, decimals + 1, '0');
    let decimal_part = &value_le[0..decimals];
    let integer_part = &value_le[decimals..decimals + 1];
    let complete = format!("{integer_part}.{decimal_part}");
    let Ok(bd) = BigDecimal::from_str_radix(&complete, 10) else {
        return None
    };
    Some(bd)
}

fn reversed_string(str: String) -> Option<String> {
    let mut bytes = str.into_bytes();
    bytes.reverse();
    String::from_utf8(bytes)
        .map(|str| Some(str))
        .unwrap_or(None)
}

fn right_pad(string: &mut String, length: usize, fill: char) {
    while string.len() < length {
        string.push(fill);
    }
}
