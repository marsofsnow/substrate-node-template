
#1、普通启动
#./target/release/node-template --dev
#2、删除已经存在的数据库后启动（通常在runtime逻辑修改后执行，runtime是区块链的运行时逻辑）
#./target/release/node-template purge-chain --dev
#3、带详细日志启动
./target/release/node-template -h
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev