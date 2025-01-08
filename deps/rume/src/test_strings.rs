#[test]
#[cfg(test)]
fn test_split() {
    use crate::strings::{split, SplitBehavior};

    assert_eq!(split("a,b,c", ",", None), vec!["a", "b", "c"]);
    assert_eq!(
        split("a,b,c", ",", Some(SplitBehavior::KeepToken)),
        vec!["a", "b", "c"]
    );
    assert_eq!(
        split("a,b,c", ",", Some(SplitBehavior::SkipToken)),
        vec!["a", "b", "c"]
    );
    assert_eq!(split("a,,c", ",", None), vec!["a", "", "c"]);
    assert_eq!(
        split("a,,c", ",", Some(SplitBehavior::KeepToken)),
        vec!["a", "", "c"]
    );
    assert_eq!(
        split("a,,c", ",", Some(SplitBehavior::SkipToken)),
        vec!["a", "c"]
    );

    assert_eq!(split("a,b,c,", ",", None), vec!["a", "b", "c"]);
    assert_eq!(
        split("a,b,c,", ",", Some(SplitBehavior::KeepToken)),
        vec!["a", "b", "c"]
    );
    assert_eq!(
        split("a,b,c,", ",", Some(SplitBehavior::SkipToken)),
        vec!["a", "b", "c"]
    );
    assert_eq!(split("a,b,c", "b", None), vec!["a,", ",c"]);
    assert_eq!(
        split("a,b,c", "b", Some(SplitBehavior::KeepToken)),
        vec!["a,", ",c"]
    );
    assert_eq!(
        split("a,b,c", "b", Some(SplitBehavior::SkipToken)),
        vec!["a,", ",c"]
    );
}
