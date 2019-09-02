use android_localization_utilities::DevExpt;
use std::cmp::Ordering;

/// Items from list1 & list2 are compared against each other using
/// the comparator & the handler would be informed about the equal
/// items
///
/// This works under a couple of assumptions
///     - Lists are sorted
///     - No repetitions in the lists
pub fn compare<ITEM1, ITEM2, COMPARATOR, HANDLER>(
    list1: &[ITEM1],
    list2: &[ITEM2],
    comparator: COMPARATOR,
    mut equal_items_handler: HANDLER,
) where
    COMPARATOR: Fn(&ITEM1, &ITEM2) -> Ordering,
    HANDLER: FnMut(&ITEM1, &ITEM2),
{
    let mut list1_index = 0;
    let mut list2_index = 0;

    // We want to exhaust both the lists in the worst case (no common items)
    let total_items_count = list1.len() + list2.len();
    for _ in 0..total_items_count {
        let item1 = list1.get(list1_index);
        let item2 = list2.get(list2_index);

        // Can't compare if either of the lists have run out! This check is imperative as the
        // code flow in the else block increments both lists' pointers if there is a match
        if item1.is_none() || item2.is_none() {
            break;
        } else {
            let item1 = item1.expt("Already checked for is_some but still fails!");;
            let item2 = item2.expt("Already checked for is_some but still fails!");;
            match comparator(item1, item2) {
                Ordering::Less => list1_index += 1,
                Ordering::Greater => list2_index += 1,
                Ordering::Equal => {
                    equal_items_handler(item1, item2);

                    // Feel free to increment both the indices as we have a `is_none` check
                    // for both the strings
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
    fn compares_with_smallest_list1() {
        let list1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let list2: Vec<f32> = vec![3.0, 6.0, 8.0];

        let mut results = vec![];
        super::compare(
            &list1,
            &list2,
            |i: &i32, f: &f32| i.cmp(&(f.clone() as i32)),
            |a, b| results.push((a.clone(), b.clone())),
        );

        assert_eq!(results, vec![(3, 3.0), (6, 6.0), (8, 8.0)])
    }

    #[test]
    fn compares_with_smallest_list2() {
        let list1: Vec<f32> = vec![3.0, 6.0, 8.0];
        let list2: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let mut results = vec![];
        super::compare(
            &list1,
            &list2,
            |f: &f32, i: &i32| (f.clone() as i32).cmp(i),
            |a, b| results.push((a.clone(), b.clone())),
        );

        assert_eq!(results, vec![(3.0, 3), (6.0, 6), (8.0, 8)])
    }
}
