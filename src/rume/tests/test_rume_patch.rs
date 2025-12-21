#[cfg(test)]
mod test {
    use crate::rume::Rume;

    #[test]
    fn test_basic() {
        let rume = Rume::new(None);

        assert_eq!(rume.apply_patch("config_id", "key", "yaml"), Ok(()));
    }
}
