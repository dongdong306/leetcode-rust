//! 模块声明由 build.rs 扫描 src/problems/*.rs 自动生成，无需手动维护。
include!(concat!(env!("OUT_DIR"), "/problem_mods.rs"));
