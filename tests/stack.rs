use hyperminhash::{SERIALIZED_SIZE, Sketch};

#[test]
fn sketch_lifecycle_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut sketch = Sketch::new();
            assert!(sketch.is_empty());
            sketch.add_many(0..1_000u64);

            let mut cloned = sketch.clone();
            assert_eq!(cloned, sketch);
            cloned.add_many(1_000..10_000u64);
            assert_ne!(cloned, sketch);

            let mut bytes = Vec::with_capacity(SERIALIZED_SIZE);
            sketch.save(&mut bytes).unwrap();
            assert_eq!(bytes.len(), SERIALIZED_SIZE);
            let loaded = Sketch::load(bytes.as_slice()).unwrap();
            assert_eq!(loaded, sketch);
            assert_eq!(
                loaded.cardinality().to_bits(),
                sketch.cardinality().to_bits()
            );

            // Exercise repeated construction and moves through an iterator without
            // placing the register buffers on the worker's stack.
            let sketches: Vec<Sketch> = (0..256u64)
                .map(|value| {
                    let mut sketch = Sketch::new();
                    sketch.add(value);
                    sketch
                })
                .collect();
            assert!(sketches.iter().all(|sketch| !sketch.is_empty()));
        })
        .expect("small-stack test thread should start")
        .join()
        .expect("sketch lifecycle should fit on a 64 KiB stack");
}
