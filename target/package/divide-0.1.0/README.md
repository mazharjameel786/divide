# Divide function
this is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`divide = "0.1.1"`

your cargo.toml file should look like this:
```
[package]
name = "divide"
version = "0.1.0"
authors = ["mazhar_jamil"]
edition = "2018"

[dependencies]
divide = "0.1.1"
```

In `src/main.rs` you can use like this:

```
use divide::arithmetic::division;
fn main() {
    println!("Hello, world!");
    division::division_res(20.0,4.0);
}
```

use divide::arithmetic::division::division_res;

```

fn main() {
    println!("Hello, world!");
    division_res(20.0,4.0);
    }
```

now `cargo run` for results
