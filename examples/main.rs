use bigdecimal::{BigDecimal, FromPrimitive};
use bigdecimal_ethers_ext::BigDecimalEthersExt;

fn main() {
    let big_decimal = BigDecimal::from_f64(10.5).unwrap();
    // Convert to an u256 with 18 decimals.
    let u256 = big_decimal.to_ethers_u256(18).unwrap();
    println!("{u256}");
}
