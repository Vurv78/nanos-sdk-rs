# ``nanos-sdk-rs`` ![CI](https://github.com/Vurv78/nanos-sdk-rs/actions/workflows/doc.yml/badge.svg) [![DocBadge](https://img.shields.io/website?down_color=red&down_message=Down&label=Docs&logo=rust&up_color=blue&up_message=Online&url=https%3A%2F%2Fvurv78.github.io%2Fnanos-sdk-rs%2Fnanos_sdk_rs)](https://vurv78.github.io/nanos-sdk-rs/nanos_sdk_rs/sdk/index.html) [![cratesio](https://img.shields.io/crates/v/nanos_sdk_rs.svg)](https://crates.io/crates/nanos-sdk-rs)

Bindings to https://github.com/nanos-world/module-sdk for Rust using bindgen.  
See **[nanos world](https://www.nanos.world)**

> Please note that nanos world is in early development, so this could become unstable. Join the discord @ [discord.nanos.world](https://discord.nanos.world)

## Usage
Add this as a dependency to your `Cargo.toml`
```toml
[dependencies]
nanos-sdk-rs = "0.1.0"
```
Then add this to build your project to a .dll
```toml
[lib]
crate-type = ["cdylib"]
```