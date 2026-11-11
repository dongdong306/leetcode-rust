# leetcode-rust

用 Rust 刷 LeetCode 的题解仓库。每道题目对应 `src/problems/` 下的一个独立文件，方法统一实现在 `crate::Solution` 上，并为每道题编写单元测试。

## 项目结构

```text
.
├── build.rs            # 扫描 src/problems/*.rs，自动生成模块注册代码
├── src/
│   ├── lib.rs          # 定义 Solution 结构体
│   └── problems/
│       ├── mod.rs      # 由 build.rs 自动生成，无需手动维护
│       └── pXXXX.rs    # 每道题一个文件
```

## 快速开始

需要较新版本的 Rust 工具链（项目使用 edition 2024）。

```bash
# 运行全部测试
cargo test

# 只运行某道题的测试，例如第 200 题
cargo test p0200

# 编译检查
cargo check
```

## 添加新题目

1. **创建题目文件** `src/problems/pXXXX.rs`：
   - 题号不足 4 位时前补零（如 `p0012.rs`），4 位及以上直接使用（如 `p3345.rs`）。
   - 文件顶部用 `//!` 文档注释写明：题号、题名、题目链接、完整题目描述、示例、提示。
   - `use crate::Solution;`，在 `impl Solution` 块中按 LeetCode 的 Rust 签名实现方法（方法名用 snake_case）。
   - 默认方法体只写 `todo!()`，留给后续自己完成。
2. **添加单元测试**：在文件底部 `#[cfg(test)] mod tests` 中根据题目示例编写测试。答案不唯一的构造题可编写验证器函数校验结果合法性，而不是断言固定输出。
3. **无需手动注册模块**：`build.rs` 会自动扫描 `src/problems/*.rs` 生成模块声明。
4. **验证**：运行 `cargo test pXXXX`。答案未实现时测试因 `todo!()` 失败属正常现象，只需确认能编译通过。

完整题目列表见 [`src/problems/`](src/problems/) 目录，每个文件顶部都有题目描述和链接。
