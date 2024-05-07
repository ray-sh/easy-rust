#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    //167 给你一个下标从 1 开始的整数数组 numbers ，该数组已按 非递减顺序排列  ，请你从数组中找出满足相加之和等于目标数 target 的两个数。如果设这两个数分别是 numbers[index1] 和 numbers[index2] ，则 1 <= index1 < index2 <= numbers.length 。

    // 以长度为 2 的整数数组 [index1, index2] 的形式返回这两个整数的下标 index1 和 index2。

    // 你可以假设每个输入 只对应唯一的答案 ，而且你 不可以 重复使用相同的元素。

    // 你所设计的解决方案必须只使用常量级的额外空间。

    fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[left] + nums[right];
            if sum == target {
                return Some((left, right));
            } else if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            }
        }
        None
    }
    #[test]
    fn test_167() {
        assert_eq!(two_sum(vec![1, 2, 3, 9], 5).unwrap(), (1, 2));
        assert_eq!(two_sum(vec![1, 2, 3, 9], 22), None);
    }
    //27 给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。

    // 不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并 原地 修改输入数组。

    // 元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。

    fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
        let (mut fast, mut slow) = (0, 0);
        while fast < nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow
    }
    #[test]
    fn test_27() {
        let mut a = vec![1, 2, 3, 1, 2];
        assert_eq!(remove_element(&mut a, 1), 3)
    }
    // 26 给你一个 升序排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。

    fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
        if nums.is_empty() {
            return 0;
        }
        let (mut slow, mut fast) = (0, 0);
        while fast < nums.len() {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }
            fast += 1;
        }
        slow + 1
    }
    #[test]
    fn test_26() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2)
    }
    // 23 给你一个链表数组，每个链表都已经按升序排列。

    // 请你将所有链表合并到一个升序链表中，返回合并后的链表。
    fn merge_lists(mut lists: Vec<LinkedList<i32>>) -> LinkedList<i32> {
        lists.iter_mut().fold(LinkedList::new(), |mut acc, list| {
            merge_two_lists(&mut acc, list)
        })
    }
    #[test]
    fn test_23() {
        assert_eq!(
            merge_lists(vec![
                LinkedList::from([1, 4, 5]),
                LinkedList::from([1, 3, 4]),
                LinkedList::from([2, 6])
            ]),
            LinkedList::from([1, 1, 2, 3, 4, 4, 5, 6])
        )
    }
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
