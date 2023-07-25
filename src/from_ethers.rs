use bigdecimal::{BigDecimal, Num};
use ethers::prelude::{I256, U256};

pub(crate) fn from_ethers_u256(bn: &U256, decimals: u8) -> Option<BigDecimal> {
    to_big_decimal(bn.to_string(), decimals)
}

pub(crate) fn from_ethers_i256(bn: &I256, decimals: u8) -> Option<BigDecimal> {
    to_big_decimal(bn.to_string(), decimals)
}

fn to_big_decimal(string: String, decimals: u8) -> Option<BigDecimal> {
    let decimals = decimals as usize;
    // e.g. for bn_str = "0", decimals = 1; must pad to "00" and return "0.0".
    let string = left_pad(string, decimals + 1, '0');
    let len = string.len();
    let decimal_start_index = len - decimals;
    let decimal_part = &string[decimal_start_index..];
    let integer_part = &string[..decimal_start_index];
    let complete = format!("{integer_part}.{decimal_part}");
    let Ok(bd) = BigDecimal::from_str_radix(&complete, 10) else {
        return None
    };
    Some(bd)
}

fn left_pad(string: String, length: usize, fill: char) -> String {
    let diff: isize = (length as isize) - (string.len() as isize);
    if diff <= 0 {
        return string;
    }
    let bytes = vec![fill as u8; diff as usize];
    let fill_string = String::from_utf8(bytes).unwrap();
    format!("{fill_string}{string}")
}

#[cfg(test)]
mod test {
    use crate::BigDecimalEthersExt;
    use bigdecimal::BigDecimal;
    use ethers::types::{I256, U256};

    #[test]
    fn should_convert_from_ethers_u256_0_decimals_0_digit() {
        let u256 = U256::from_str_radix("", 10).unwrap();
        let bd = BigDecimal::from_ethers_u256(&u256, 0).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "0");
    }

    #[test]
    fn should_convert_from_ethers_u256_0_decimals_1_digit() {
        let u256 = U256::from_str_radix("3", 10).unwrap();
        let bd = BigDecimal::from_ethers_u256(&u256, 0).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "3");
    }

    #[test]
    fn should_convert_from_ethers_i256_0_decimals_1_digit() {
        let i256 = I256::from_dec_str("-3").unwrap();
        let bd = BigDecimal::from_ethers_i256(&i256, 0).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "-3");
    }

    #[test]
    fn should_convert_from_ethers_u256_0_decimals_2_digit() {
        let u256 = U256::from_str_radix("33", 10).unwrap();
        let bd = BigDecimal::from_ethers_u256(&u256, 0).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "33");
    }

    #[test]
    fn should_convert_from_ethers_i256_0_decimals_2_digit() {
        let i256 = I256::from_dec_str("-33").unwrap();
        let bd = BigDecimal::from_ethers_i256(&i256, 0).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "-33");
    }

    #[test]
    fn should_convert_from_ethers_u256_0_decimals_4_digits() {
        let u256 = U256::from_str_radix("1005", 10).unwrap();
        let bd = BigDecimal::from_ethers_u256(&u256, 0).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "1005");
    }

    #[test]
    fn should_convert_from_ethers_i256_0_decimals_4_digits() {
        let i256 = I256::from_dec_str("-1005").unwrap();
        let bd = BigDecimal::from_ethers_i256(&i256, 0).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "-1005");
    }

    #[test]
    fn should_convert_from_ethers_u256_1_decimal_1_digit() {
        let u256 = U256::from_str_radix("3", 10).unwrap();
        let bd = BigDecimal::from_ethers_u256(&u256, 1).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "0.3");
    }

    #[test]
    fn should_convert_from_ethers_i256_1_decimal_1_digit() {
        let i256 = I256::from_dec_str("-3").unwrap();
        let bd = BigDecimal::from_ethers_i256(&i256, 1).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "-0.3");
    }

    #[test]
    fn should_convert_from_ethers_u256_1_decimal_2_digit() {
        let u256 = U256::from_str_radix("33", 10).unwrap();
        let bd = BigDecimal::from_ethers_u256(&u256, 1).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "3.3");
    }

    #[test]
    fn should_convert_from_ethers_i256_1_decimal_2_digit() {
        let i256 = I256::from_dec_str("-33").unwrap();
        let bd = BigDecimal::from_ethers_i256(&i256, 1).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "-3.3");
    }

    #[test]
    fn should_convert_from_ethers_u256_1_decimal_4_digits() {
        let u256 = U256::from_str_radix("1005", 10).unwrap();
        let bd = BigDecimal::from_ethers_u256(&u256, 1).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "100.5");
    }

    #[test]
    fn should_convert_from_ethers_i256_1_decimal_4_digits() {
        let i256 = I256::from_dec_str("-1005").unwrap();
        let bd = BigDecimal::from_ethers_i256(&i256, 1).unwrap();
        let bd_string = bd.to_string();
        assert_eq!(bd_string, "-100.5");
    }
}
