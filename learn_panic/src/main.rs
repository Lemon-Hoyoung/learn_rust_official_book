// 默认 panic发生的时候：
// 1. 程序展开调用栈（Rust沿着调用栈往回走，并清理每个遇到的函数中的数据）
// 2. 立即中止调用栈（不清理，直接停止程序，由操作系统来清理）（Cargo.toml中设置panic = 'abort'）

fn main() {
    // 使用panic!（panic宏）来直接产生panic
    // panic!("Crash and burn");

    // 使用RUST_BACKTRACE=1 cargo run调用执行能够看到详细的panic调用栈
    let v = vec![1,2,3];
    v[10];
}
