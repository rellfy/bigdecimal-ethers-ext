# bigdecimal-ethers-ext
[BigDecimal][0] interoperability with [ethers-rs][1] types.

# Example
```rs
let big_decimal = BigDecimal::from_f64(10.5).unwrap();
// Convert to an u256 with 18 decimals.
let u256 = big_decimal.to_ethers_u256(18).unwrap();
println!("{u256}");
// 10500000000000000000
```

[0]: https://github.com/akubera/bigdecimal-rs
[1]: https://github.com/gakonst/ethers-rs
