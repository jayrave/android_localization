use std::fmt::Debug;

pub fn assert_list_is_empty<T, L>(list: L)
where
    T: Debug,
    L: AsRef<[T]> + Debug,
{
    if !list.as_ref().is_empty() {
        panic!("Expected list to be empty: {:?}", list)
    }
}

pub fn assert_strict_list_eq<T, S, LT, LS>(list1: LT, list2: LS)
where
    T: PartialEq<S> + Debug,
    S: Debug,
    LT: AsRef<[T]> + Debug,
    LS: AsRef<[S]> + Debug,
{
    list1
        .as_ref()
        .iter()
        .zip(list2.as_ref())
        .for_each(|items: (&T, &S)| {
            if items.0 != items.1 {
                panic!(
                    r#"Expected items to be equal
Item 1: {:?}
Item 2: {:?}
List 1: {:?}
List 2: {:?}"#,
                    items.0, items.1, list1, list2
                )
            }
        })
}
