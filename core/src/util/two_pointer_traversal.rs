use std::cmp::Ordering;

use android_localization_utilities::DevExpt;

/// Look @ the docs for [traverse]
pub fn compare<ITEM1, ITEM2, COMPARATOR, HANDLER>(
    list1: &[ITEM1],
    list2: &[ITEM2],
    comparator: COMPARATOR,
    equal_items_handler: HANDLER,
) where
    COMPARATOR: Fn(&ITEM1, &ITEM2) -> Ordering,
    HANDLER: FnMut(&ITEM1, &ITEM2),
{
    // Each closure has a unique signature & so need to have these
    // dummy ones to help the compiler
    let extra1_handler = |_: &ITEM1| {};
    let extra2_handler = |_: &ITEM2| {};
    traverse(
        list1,
        list2,
        comparator,
        Some(equal_items_handler),
        Some(extra1_handler),
        Some(extra2_handler),
    )
}

/// Look @ the docs for [traverse]
pub fn diff<ITEM1, ITEM2, COMPARATOR, ExtraInList1Hander, ExtraInList2Hander>(
    list1: &[ITEM1],
    list2: &[ITEM2],
    comparator: COMPARATOR,
    extra_in_list1_handler: ExtraInList1Hander,
    extra_in_list2_handler: ExtraInList2Hander,
) where
    COMPARATOR: Fn(&ITEM1, &ITEM2) -> Ordering,
    ExtraInList1Hander: FnMut(&ITEM1),
    ExtraInList2Hander: FnMut(&ITEM2),
{
    // Each closure has a unique signature & so need to have this
    // dummy ones to help the compiler
    let equal_handler = |_: &ITEM1, _: &ITEM2| {};
    traverse(
        list1,
        list2,
        comparator,
        Some(equal_handler),
        Some(extra_in_list1_handler),
        Some(extra_in_list2_handler),
    )
}

/// Items from list1 & list2 are compared against each other using the
/// comparator & the handlers would be informed about the appropriate items
///
/// This works under a couple of assumptions
///     - Lists are sorted
///     - No repetitions in the lists
fn traverse<ITEM1, ITEM2, COMPARATOR, EqualHandler, ExtraInList1Hander, ExtraInList2Hander>(
    list1: &[ITEM1],
    list2: &[ITEM2],
    comparator: COMPARATOR,
    mut equal_items_handler: Option<EqualHandler>,
    mut extra_in_list1_handler: Option<ExtraInList1Hander>,
    mut extra_in_list2_handler: Option<ExtraInList2Hander>,
) where
    COMPARATOR: Fn(&ITEM1, &ITEM2) -> Ordering,
    EqualHandler: FnMut(&ITEM1, &ITEM2),
    ExtraInList1Hander: FnMut(&ITEM1),
    ExtraInList2Hander: FnMut(&ITEM2),
{
    let mut list1_index = 0;
    let mut list2_index = 0;
    let message_expected_some = "Already checked for is_some but still fails!";

    // We want to exhaust both the lists in the worst case (no common items)
    let total_items_count = list1.len() + list2.len();
    for _ in 0..total_items_count {
        let item1 = list1.get(list1_index);
        let item2 = list2.get(list2_index);

        // Both the lists have run out! Finish
        if item1.is_none() && item2.is_none() {
            break;
        } else {
            let ordering = if item1.is_some() && item2.is_none() {
                Ordering::Less
            } else if item1.is_none() && item2.is_some() {
                Ordering::Greater
            } else {
                let item1 = item1.expt(message_expected_some);
                let item2 = item2.expt(message_expected_some);
                comparator(item1, item2)
            };

            match ordering {
                Ordering::Less => {
                    if let Some(ref mut handler) = extra_in_list1_handler {
                        handler(item1.expt(message_expected_some));
                    }

                    list1_index += 1
                }

                Ordering::Greater => {
                    if let Some(ref mut handler) = extra_in_list2_handler {
                        handler(item2.expt(message_expected_some));
                    }

                    list2_index += 1
                }

                Ordering::Equal => {
                    if let Some(ref mut handler) = equal_items_handler {
                        handler(
                            item1.expt(message_expected_some),
                            item2.expt(message_expected_some),
                        );
                    }

                    list1_index += 1;
                    list2_index += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn common_and_extra_elements1() {
        let list1: Vec<i32> = vec![1, 2, 3, 4, 6, 7, 8];
        let list2: Vec<f32> = vec![-1.0, 3.0, 5.0, 8.0, 9.0];

        let mut common = vec![];
        let mut only_in_list1 = vec![];
        let mut only_in_list2 = vec![];

        super::traverse(
            &list1,
            &list2,
            |i: &i32, f: &f32| i.cmp(&(f.clone() as i32)),
            Some(|i: &i32, f: &f32| common.push((i.clone(), f.clone()))),
            Some(|i: &i32| only_in_list1.push(i.clone())),
            Some(|f: &f32| only_in_list2.push(f.clone())),
        );

        assert_eq!(common, vec![(3, 3.0), (8, 8.0)]);
        assert_eq!(only_in_list1, vec![1, 2, 4, 6, 7]);
        assert_eq!(only_in_list2, vec![-1.0, 5.0, 9.0])
    }

    #[test]
    fn common_and_extra_elements2() {
        let list1: Vec<f32> = vec![-1.0, 3.0, 5.0, 8.0, 9.0];
        let list2: Vec<i32> = vec![1, 2, 3, 4, 6, 7, 8];

        let mut common = vec![];
        let mut only_in_list1 = vec![];
        let mut only_in_list2 = vec![];

        super::traverse(
            &list1,
            &list2,
            |f: &f32, i: &i32| (f.clone() as i32).cmp(i),
            Some(|f: &f32, i: &i32| common.push((f.clone(), i.clone()))),
            Some(|f: &f32| only_in_list1.push(f.clone())),
            Some(|i: &i32| only_in_list2.push(i.clone())),
        );

        assert_eq!(common, vec![(3.0, 3), (8.0, 8)]);
        assert_eq!(only_in_list1, vec![-1.0, 5.0, 9.0]);
        assert_eq!(only_in_list2, vec![1, 2, 4, 6, 7])
    }

    #[test]
    fn only_common_elements() {
        let list1: Vec<i32> = vec![1, 2, 3];
        let list2: Vec<f32> = vec![1.0, 2.0, 3.0];

        let mut common = vec![];
        let mut only_in_list1 = vec![];
        let mut only_in_list2 = vec![];

        super::traverse(
            &list1,
            &list2,
            |i: &i32, f: &f32| i.cmp(&(f.clone() as i32)),
            Some(|i: &i32, f: &f32| common.push((i.clone(), f.clone()))),
            Some(|i: &i32| only_in_list1.push(i.clone())),
            Some(|f: &f32| only_in_list2.push(f.clone())),
        );

        assert_eq!(common, vec![(1, 1.0), (2, 2.0), (3, 3.0)]);
        assert_eq!(only_in_list1, vec![]);
        assert_eq!(only_in_list2, vec![])
    }

    #[test]
    fn only_extra_elements() {
        let list1: Vec<i32> = vec![1, 2, 3];
        let list2: Vec<f32> = vec![4.0, 5.0, 6.0];

        let mut common = vec![];
        let mut only_in_list1 = vec![];
        let mut only_in_list2 = vec![];

        super::traverse(
            &list1,
            &list2,
            |i: &i32, f: &f32| i.cmp(&(f.clone() as i32)),
            Some(|i: &i32, f: &f32| common.push((i.clone(), f.clone()))),
            Some(|i: &i32| only_in_list1.push(i.clone())),
            Some(|f: &f32| only_in_list2.push(f.clone())),
        );

        assert_eq!(common, vec![]);
        assert_eq!(only_in_list1, vec![1, 2, 3]);
        assert_eq!(only_in_list2, vec![4.0, 5.0, 6.0]);
    }
}
