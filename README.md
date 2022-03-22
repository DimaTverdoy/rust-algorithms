<img src="https://miro.medium.com/max/1400/1*-EFdnPuVrwUOmYte11v0OA.png">

# Rust data structures and algorithms

Minimal example implementations of data structures and algorithms in rust.

## Doc
    # cargo doc --open

## Tests
    $ cargo test

## Use
```rust
use rust_algoritms::sort::insert::sort;

fn main() {
    let mut x = [1, 43, 52, 5, 0];
    
    sort(&mut x);
    assert_eq!(x, [0, 1, 5, 43, 52]);
}
```

## Link

https://algorist.com - basically all algorithms are taken from here