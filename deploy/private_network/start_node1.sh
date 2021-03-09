../../target/debug/node-template \
  --base-path ../node1 \
  --chain ./customSpecRaw.json \
  --port 40333 \
  --ws-port 49944 \
  --rpc-port 49933 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --rpc-methods=Unsafe \
  --name node1