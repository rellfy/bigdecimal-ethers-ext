use bigdecimal::{BigDecimal, FromPrimitive};
use bigdecimal_ethers_ext::BigDecimalEthersExt;

fn main() {
    let big_decimal = BigDecimal::from_f64(10.5).unwrap();
    // Convert to an u256 with 18 decimals.
    let u256 = big_decimal.to_ethers_u256(18).unwrap();
    println!("{u256}");
    // 10500000000000000000
    // Convert from an u256 with 18 decimals to a BigDecimal.
    let to_big_decimal = BigDecimal::from_ethers_u256(&u256, 18).unwrap();
    println!("{to_big_decimal}");
    // 10.500000000000000000
}
