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

    // 876. 题「链表的中间结点

    // 」就是这个题目，问题的关键也在于我们无法直接得到单链表的长度 n，常规方法也是先遍历链表计算 n，再遍历一次得到第 n / 2 个节点，也就是中间节点。

    // 如果想一次遍历就得到中间节点，也需要耍点小聪明，使用「快慢指针」的技巧：

    // 我们让两个指针 slow 和 fast 分别指向链表头结点 head。

    // 每当慢指针 slow 前进一步，快指针 fast 就前进两步，这样，当 fast 走到链表末尾时，slow 就指向了链表中点。
    fn middle_node(list: &LinkedList<i32>) -> Option<&i32> {
        let mut iter1 = list.iter();
        let mut iter2 = list.iter();
        loop {
            iter2.next();
            if iter2.next().is_none() {
                return iter1.next();
            }
            iter1.next();
        }
    }
    #[test]
    fn test_876() {
        let l1 = LinkedList::from([1, 2, 4]);
        assert_eq!(middle_node(&l1), Some(2).as_ref());
    }
}
