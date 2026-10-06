#[test]
fn deserialize_is_stable() {
    let serialized = include_bytes!("serialized.bin");
    assert_eq!(serialized.len(), hyperminhash::SERIALIZED_SIZE);

    // Ensure that a Sketch serialized once yields the same result forever...
    // Explicit type arguments were supported by the 0.1.x API.
    let sketch = hyperminhash::Sketch::load::<&[u8]>(&serialized[..]).unwrap();
    assert_eq!(sketch.cardinality(), 9931.106244547593);

    let mut saved = Vec::with_capacity(hyperminhash::SERIALIZED_SIZE);
    sketch.save::<&mut Vec<u8>>(&mut saved).unwrap();
    assert_eq!(saved.as_slice(), serialized.as_slice());
}
