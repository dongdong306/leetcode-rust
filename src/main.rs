use leetcode_rust::Solution;

fn main() {
    let res = Solution::read_binary_watch(4);
    for r in res {
        println!("{:?}", r);
    }
}
