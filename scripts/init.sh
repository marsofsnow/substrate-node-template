# add new
# 初始化编译环境，包括升级Rust的版本，包括nightly和stable两个发布渠道：
rustup update nightly
rustup update stable
# 添加构建WebAssembly的支持工具：
rustup target add wasm32-unknown-unknown --toolchain nightly

# 定期执行本脚本，可以解决一些常见的编译问题如某个依赖安装失败。