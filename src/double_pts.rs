#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    // 21. 合并两个有序链表
    // 输入：l1 = [1,2,4], l2 = [1,3,4]
    fn merge_two_lists(l1: &mut LinkedList<i32>, l2: &mut LinkedList<i32>) -> LinkedList<i32> {
        let mut result = LinkedList::new();
        loop {
            match (l1.front(), l2.front()) {
                (Some(vl1), Some(vl2)) => {
                    if vl1 <= vl2 {
                        result.push_back(l1.pop_front().unwrap());
                    } else {
                        result.push_back(l2.pop_front().unwrap());
                    }
                }
                (Some(_), None) => {
                    result.push_back(l1.pop_front().unwrap());
                }
                (None, Some(_)) => {
                    result.push_back(l2.pop_front().unwrap());
                }
                (None, None) => return result,
            }
        }
    }
    #[test]
    fn test_21() {
        let mut l1 = LinkedList::from([1, 2, 4]);
        let mut l2 = LinkedList::from([1, 3, 4, 5, 6]);
        assert_eq!(
            merge_two_lists(&mut l1, &mut l2),
            LinkedList::from([1, 1, 2, 3, 4, 4, 5, 6])
        );
    }
}
