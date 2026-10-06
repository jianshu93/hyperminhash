#![cfg(feature = "serialize")]

use hyperminhash::{SERIALIZED_SIZE, Sketch};

#[test]
fn json_round_trip_preserves_registers() {
    for sketch in [Sketch::new(), (0..10_000u64).collect()] {
        let json = serde_json::to_string(&sketch).unwrap();
        let registers: Vec<u16> = serde_json::from_str(&json).unwrap();
        assert_eq!(registers.len(), SERIALIZED_SIZE / 2);
        let loaded: Sketch = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded, sketch);
    }
}

#[test]
fn bincode_matches_legacy_binary_format() {
    let fixture = include_bytes!("serialized.bin");
    let sketch = Sketch::load(fixture.as_slice()).unwrap();
    let encoded = bincode::serialize(&sketch).unwrap();
    assert_eq!(encoded.as_slice(), fixture.as_slice());
    let loaded: Sketch = bincode::deserialize(fixture).unwrap();
    assert_eq!(loaded, sketch);

    // Nested values must leave the next sketch's registers for its own visitor.
    let sketches = vec![sketch, Sketch::new()];
    let encoded = bincode::serialize(&sketches).unwrap();
    let loaded: Vec<Sketch> = bincode::deserialize(&encoded).unwrap();
    assert_eq!(loaded, sketches);
}

#[test]
fn rejects_wrong_register_counts_and_values() {
    for len in [0, SERIALIZED_SIZE / 2 - 1, SERIALIZED_SIZE / 2 + 1] {
        let json = serde_json::to_string(&vec![0u16; len]).unwrap();
        assert!(serde_json::from_str::<Sketch>(&json).is_err());
    }
    for value in ["-1", "65536", "1.5", "null", "\"invalid\""] {
        let json = format!("[{value},{}]", vec!["0"; SERIALIZED_SIZE / 2 - 1].join(","));
        assert!(serde_json::from_str::<Sketch>(&json).is_err());
    }
    assert!(bincode::deserialize::<Sketch>(&vec![0; SERIALIZED_SIZE - 1]).is_err());
}

#[test]
fn serde_round_trips_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let sketch: Sketch = (0..1_000u64).collect();
            let bytes = bincode::serialize(&sketch).unwrap();
            let loaded: Sketch = bincode::deserialize(&bytes).unwrap();
            assert_eq!(loaded, sketch);
            let json = serde_json::to_string(&sketch).unwrap();
            let loaded: Sketch = serde_json::from_str(&json).unwrap();
            assert_eq!(loaded, sketch);
        })
        .expect("small-stack test thread should start")
        .join()
        .expect("Serde round trips should fit on a 64 KiB stack");
}
