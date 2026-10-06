# Hyperminhash for Rust

[![Crates.io Version](https://img.shields.io/crates/v/hyperminhash.svg)](https://crates.io/crates/hyperminhash)
[![Docs](https://docs.rs/hyperminhash/badge.svg)](https://docs.rs/hyperminhash)
[![PyPI](https://badge.fury.io/py/pyhyperminhash.svg)](https://pypi.org/project/pyhyperminhash/)

A straight port of [Hyperminhash](https://github.com/axiomhq/hyperminhash) for Rust.
Very fast, constant memory-footprint cardinality approximation,
including intersection and union operation.

```rust
use std::{io, io::Bufread, fs};

let reader = io::BufReader::new(fs::File::open(fname)?).lines();
let sketch = reader.collect::<io::Result<hyperminhash::Sketch>>()?;
println!("{}", sketch.cardinality());
```

### Serialization

`Sketch::save` and `Sketch::load` are always available. The `serialize` Cargo
feature uses the same name as 0.1.x and enables Serde support:

```toml
hyperminhash = { version = "0.2.5", features = ["serialize"] }
```

The binary format is unchanged from 0.1.3: 16,384 little-endian `u16` registers
(32,768 bytes), without a header. Register storage is heap-allocated, including
when loading a sketch. Files written by 0.1.3 can be read by 0.2.5 and vice versa.

Enable the `serialize` feature for `serde::Serialize` and
`serde::Deserialize` implementations on `Sketch`:

```toml
hyperminhash = { version = "0.2.5", features = ["serialize"] }
serde_json = "1"
```

```rust
use hyperminhash::Sketch;

fn main() -> Result<(), serde_json::Error> {
    let sketch: Sketch = (0..1_000u64).collect();
    let json = serde_json::to_string(&sketch)?;
    let restored: Sketch = serde_json::from_str(&json)?;
    assert_eq!(restored, sketch);
    Ok(())
}
```

Serde represents a sketch as a fixed-length tuple of 16,384 `u16` registers
(an array in JSON). Deserialization validates the length and fills a
heap-allocated buffer. With bincode 1.3's `serialize`/`deserialize` functions,
the bytes match `Sketch::save`/`Sketch::load` exactly. Other formats or bincode
configurations may use different encodings.

#### Two files of 10,000,000 random strings each:

Operation | Runtime | Result
----------|----------------|-------
Cardinality via `sort strings1.txt \| uniq \| wc -l` | 7.01 secs | 9,779,544
Union via `cat strings1.txt string2.txt \| sort \| uniq \| wc -l` | 16.19 secs | 19,130,942
Intersection via `comm -12 <(sort string1.txt) <(sort strings2.txt) \| wc -l` | 6.67 secs | 428,568
Cardinality via Hyperminhash | 0.45 secs | 9,792,822
Cardinality via Hyperminhash ([multithreaded](https://github.com/lukaslueg/hyperminhash/blob/master/examples/parallel.rs)) | 0.29 secs | 9,792,822
Union via Hyperminhash | 0.44 secs | 19,268,781
Intersection via Hyperminhash | 0.44 secs | 434,141
