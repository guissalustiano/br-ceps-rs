# br-ceps

A blazing fast, lightweight Rust library for resolving Brazilian postal codes (CEP).

Network it's complicate, just solve it locally with the database as crate. 

Don't forget to keep the trait update, it's update montly.


## Installation
Add with cargo
```bash
cargo add br-ceps
```

Or add manually `Cargo.toml`:
```toml
[dependencies]
br-ceps = "0.0.1"
```

## Usage

```rust
use br_ceps::get;

fn main() {
    // Look up a CEP
    let result = get("01246904").unwrap();
    
    // Access the returned data
    println!("CEP: {}", result.cep);
    println!("State: {}", result.uf); 
    println!("City: {}", result.localidade); 
    println!("Street: {}", result.logradouro); 
}
```
