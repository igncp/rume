use super::{parse_prism_file, PRISM_FORMAT_PREFIX};

#[test]
fn parses_bopomofo_prism_bin_metadata() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/data/test/bopomofo.prism.bin", manifest_dir);

    let info = parse_prism_file(&path).expect("failed to parse prism bin");

    assert!(info.format.starts_with(PRISM_FORMAT_PREFIX));
    assert!(
        info.version >= 1.0,
        "unexpected prism version: {}",
        info.version
    );
    assert!(info.num_syllables > 0, "no syllables reported");
    assert!(info.double_array_size > 0, "double array size must be > 0");
    assert!(
        info.double_array_offset_abs.is_some(),
        "missing double array blob"
    );
    assert!(!info.alphabet.is_empty(), "alphabet should not be empty");
}
