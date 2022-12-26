# is_copy
Determine whether two files are copies of each other

# Usage
Add this to your *Cargo.toml*:
```toml
[dependencies]
is_copy = "0.1"
```

## Examples
Create and determin file using is_copy:

```rust
#[macro_use]
use is_copy::is_file_copy;

let path_a = Path::new("./data/file_a.txt");
let path_b = Path::new("./data/file_b.txt");

let is_copy = is_file_copy(path_a, path_b);
println!("{:?} and {:?} is copy: {}", is_copy);
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.