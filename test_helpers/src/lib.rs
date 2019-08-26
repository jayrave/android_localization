use std::fmt::Debug;

pub fn assert_eq_to_either_or<T>(actual: T, expected1: T, expected2: T)
    where T: PartialEq + Debug
{
    assert_eq_to_either_or_by(actual, expected1, expected2, |item1, item2| item1 == item2)
}

pub fn assert_eq_to_either_or_by<T, F>(actual: T, expected1: T, expected2: T, comparator: F)
    where
        T: Debug,
        F: Fn(&T, &T) -> bool,
{
    let result1 = comparator(&actual, &expected1);
    let result2 = comparator(&actual, &expected2);
    assert!(
        result1 || result2,
        r#"---------
Actual
{:?}
Expected either
{:?}
or
{:?}
---------"#,
        actual,
        expected1,
        expected2
    )
}