# StrEnc

"Hides" static strings from the compiled binary, so they are not "human readable".

Lib strongly inspirited by [LITCRYPT](https://crates.io/crates/litcrypt). <br>
Click for more details.

## Usage
Add a dependencies in `Cargo.toml`
```toml
[dependencies.strenc]
git = "https://github.com/DmitrijVC/strenc"

[dependencies]
magic-crypt = "3.1.8"
```

## Example
```rust
#[macro_use] extern crate strenc;
#[macro_use] extern crate magic_crypt;  // StrEnc requires magic_crypt

strenc_initialize!();  // one-time lib initialization


fn main() {
    // this string won't be visible in the binary
    let x: String = enc!("ConfidentialData");

    // this string will be visible in the binary
    let a = String::new("NotConfidentialData");

    println!(
        "[{}]: <{:?}>", 
        &x,
        &x as *const String
    );
}
```
 
## Differences from LITCRYPT
- Different encryption method (requires magic_crypt)
- Random 32 len key generated for every string

## Disclaimer
It's my first time doing something with macros and this lib has been made for my private project. Consider using LITCRYPT instead, as it should be more efficient.
