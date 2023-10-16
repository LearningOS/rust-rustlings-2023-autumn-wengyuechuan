//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置环境变量 TEST_FOO 为 timestamp 的值
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 在 tests8 中需要进行额外的配置，所以需要添加相应的代码
    #[cfg(feature = "pass")]
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
