RUST_LOG=warn RUST_BACKTRACE=1  cargo run  --bin node-template --  --dev  -lruntime=debug   -d ./deploy/hello > ./deploy/1.log

# ./target/release/node-template purge-chain --dev