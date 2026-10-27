use leetcode_rust::Solution;

fn main() {
    let board = vec![
        vec!['A', 'B'],
        vec!['C', 'D'],
    ];

    let res = Solution::exist(board.clone(), "DBAC".to_string());
    println!(" -> {}", res);

}
