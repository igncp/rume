#[cfg(test)]
use super::rume_patch;

#[cfg(test)]
mod test {
    use chrono::{DateTime, Local, TimeZone};
    use rume::rume::version::RUME_VERSION;

    use super::*;

    #[test]
    fn test_apply_patch1() {
        let file_name = "test_apply_patch1_test.custom.yaml";

        let remove_file = || {
            std::fs::remove_file(file_name).unwrap_or((
                // Ignore if the file does not exist
            ));
        };

        remove_file();

        let current_time =
            DateTime::parse_from_str("2025-06-13 19:14:44 +0800", "%Y-%m-%d %H:%M:%S %z")
                .expect("Failed to parse date");

        rume_patch(
            "test_apply_patch1_test",
            "foo",
            "bar".to_string(),
            Some(Local.from_utc_datetime(&current_time.naive_utc())),
        )
        .expect("Failed to apply patch");

        let file_content = std::fs::read_to_string(file_name).expect("Failed to read patch file");

        assert_eq!(
            file_content,
            format!(
                r#"
customization:
  distribution_code_name: ""
  distribution_version: ""
  generator: rume_patch
  modified_time: "Fri Jun 13 19:14:44 2025"
  rume_version: {RUME_VERSION}
patch:
  foo: bar"#
            )
            .trim()
        );

        remove_file();
    }
}
