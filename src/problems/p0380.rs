//! LeetCode 第380题：O(1) 时间插入、删除和获取随机元素
//!
//! 题目链接：https://leetcode.cn/problems/insert-delete-getrandom-o1/
//!
//! 实现 RandomizedSet 类：
//!
//! - RandomizedSet() 初始化 RandomizedSet 对象
//! - bool insert(int val) 当元素 val 不存在时，向集合中插入该项，并返回 true ；否则，返回 false 。
//! - bool remove(int val) 当元素 val 存在时，从集合中移除该项，并返回 true ；否则，返回 false 。
//! - int getRandom() 随机返回现有集合中的一项（测试用例保证调用此方法时集合中至少存在一个元素）。每个元素应该有 相同的概率 被返回。
//!
//! 你必须实现类的所有函数，并满足每个函数的 平均 时间复杂度为 O(1) 。
//!
//! ## 示例
//!
//! 输入：
//! - ["RandomizedSet","insert","remove","insert","getRandom","remove","insert","getRandom"]
//! - [[],[1],[2],[2],[],[1],[2],[]]
//!
//! 输出：[null,true,false,true,2,true,false,2]
//!
//! 解释：
//! - RandomizedSet randomizedSet = new RandomizedSet();
//! - randomizedSet.insert(1); // 向集合中插入 1 ，返回 true
//! - randomizedSet.remove(2); // 集合中不存在 2 ，返回 false
//! - randomizedSet.insert(2); // 向集合中插入 2 ，返回 true ，集合现在包含 [1, 2]
//! - randomizedSet.getRandom(); // getRandom 应随机返回 1 或 2
//! - randomizedSet.remove(1); // 从集合中移除 1 ，返回 true ，集合现在包含 [2]
//! - randomizedSet.insert(2); // 集合中已存在 2 ，返回 false
//! - randomizedSet.getRandom(); // 由于 2 是集合中唯一的项目，getRandom 总是返回 2
//!
//! ## 提示
//!
//! - -2^31 <= val <= 2^31 - 1
//! - 最多调用 insert、remove 和 getRandom 函数 2 * 10^5 次
//! - 调用 getRandom 方法时，数据集合中 至少 存在一个元素

use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;

struct RandomizedSet {
    array: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        return RandomizedSet{
            array: Vec::new(),
            map: HashMap::new(),
        };
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.map.insert(val, self.array.len());
            self.array.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            Some(index) => {
                let last_index = self.array.len() - 1;
                self.array.swap_remove(index);
                if last_index != index {
                    let last_value = self.array[index];
                    self.map.insert(last_value, index);
                }

                true
            }
            None => false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.array.choose(&mut rng).expect("RandomizedSet is empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));

        let random = set.get_random();
        assert!(random == 1 || random == 2);

        assert!(set.remove(1));
        assert!(!set.insert(2));
        assert_eq!(set.get_random(), 2);
    }
}
