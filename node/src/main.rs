//! Substrate Node Template CLI library.
#![warn(missing_docs)] //在编译时，如果模块缺少文档会打印警告信息

//引入当前目录下的其他模块
mod chain_spec;
#[macro_use] //加载引入的模块下的所有宏
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
