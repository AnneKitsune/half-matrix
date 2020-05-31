Support an Open Source Developer! :hearts:  

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

# Half Matrix
A Half Matrix implementation. Also called a triangular matrix.
Like a normal matrix, but you only store half of it.

Used to define relations between two elements of the same set.

This can be used to represent graphs (an edge between point A and B).
It can also be used to represent associations (group A should have physical collisions enabled
with group A and group B).

## Usage

Cargo.toml:
```toml
[dependencies]
half_matrix = "*"
```
Code:
```rust
// 3x3
//
//  012
// 2oxo
// 1xx
// 0o

let mut m = HalfMatrix::new(3);
m.enable(2, 0);
m.enable(2, 2);
m.enable(0, 0);

assert!(m.contains(2,0));
assert!(m.contains(2,2));
assert!(m.contains(0,0));

assert!(!m.contains(1,0));
assert!(!m.contains(1,1));
assert!(!m.contains(2,1));
```

