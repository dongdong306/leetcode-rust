# leetcode-rust 项目说明

Rust 刷题项目。每道 LeetCode 题目对应 `src/problems/` 下的一个独立文件，方法统一实现在 `crate::Solution` 上（定义于 `src/lib.rs`）。

## 添加新题目的流程

1. **获取题目信息**：从 LeetCode 题目链接抓取题目页面，提取题号、题名、题目描述、示例和提示。

2. **创建题目文件** `src/problems/pXXXX.rs`：
   - 文件命名规则：题号不足 4 位时前补零（如 `p0012.rs`），4 位及以上直接使用（如 `p3345.rs`）。
   - 文件结构：
     - 顶部 `//!` 文档注释：题号、题名、题目链接、完整题目描述、示例、提示。
     - `use crate::Solution;`
     - `impl Solution` 块，包含方法签名（按 LeetCode Rust 签名，方法名用 snake_case）。
   - **默认不要写答案**：方法体只写 `todo!()`，留给用户自己完成（除非用户明确要求写答案）。

3. **添加单元测试**（可选但推荐）：在文件底部 `#[cfg(test)] mod tests` 中根据题目示例编写测试。对于"答案不唯一"的构造题，可编写验证器函数校验结果合法性，而不是断言固定输出。

4. **注册模块**：在 `src/problems/mod.rs` 中添加 `pub mod pXXXX;`（保持追加到文件末尾的习惯）。

5. **验证**：运行 `cargo test pXXXX`。答案未实现时测试因 `todo!()` 失败属正常现象，只需确认能编译通过。
