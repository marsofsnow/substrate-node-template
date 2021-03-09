Substrate提供Aura、BABE、PoW等多种生产区块的算法，
sr25519格式的密钥及其关联的SS58地址会被Aura使用。

1. 使用subkey generate命令生成两组sr25519格式密钥和SS58地址，我习惯加上--output-type json选项让输出看起来更清楚：
```
subkey generate --output-type json
{
  "accountId": "0x92ee9ede7c86a968d09860636ae80ae44e4b13e2626a239401c0146589e23a63",
  "publicKey": "0x92ee9ede7c86a968d09860636ae80ae44e4b13e2626a239401c0146589e23a63",
  "secretPhrase": "fortune churn emerge mushroom shoe rally valley fat can cute parent curious",
  "secretSeed": "0x56dadb21dafec61bc08641f0bb76cfe5e56a566a29b59571d2a986a523dd9db0",
  "ss58Address": "5FPMjVZstELDEgdYRc9vhK3bYzRLFKsQZTTy5usuY5X1xJY7"
}

 subkey inspect --scheme ed25519 --output-type json "fortune churn emerge mushroom shoe rally valley fat can cute parent curious"
{
  "accountId": "0x59e4135a22883ba80ce99d1b688e27ea0aa8c6d91f860603408d5073f8312288",
  "publicKey": "0x59e4135a22883ba80ce99d1b688e27ea0aa8c6d91f860603408d5073f8312288",
  "secretPhrase": "fortune churn emerge mushroom shoe rally valley fat can cute parent curious",
  "secretSeed": "0x56dadb21dafec61bc08641f0bb76cfe5e56a566a29b59571d2a986a523dd9db0",
  "ss58Address": "5E6ZsVzG9pVgshmxWTjQYhvhM716NcJLEXUNXjXCwdUN3DxB"
}



 subkey generate --output-type json
{
  "accountId": "0x22a7d399e696f1337c2d84851ab746f980d7582f681c659e0b1b0b253594f74b",
  "publicKey": "0x22a7d399e696f1337c2d84851ab746f980d7582f681c659e0b1b0b253594f74b",
  "secretPhrase": "day lumber clip wear century elite light rival barely relief tag pledge",
  "secretSeed": "0xcf015e0ddfc47d1897e0c09ea67d9d5edd8d5e8f9d1683032bace30e33ae6bc6",
  "ss58Address": "5Cr9LgVuAQpjA5k9gpWH6AcnSwys1kbMC2UC1Qd7rNE7rbCi"
}

subkey inspect --scheme ed25519 --output-type json "day lumber clip wear century elite light rival barely relief tag pledge"
{
  "accountId": "0x8eb995b11838c69702f5050964990ec89bf8452440c432796e30292330983817",
  "publicKey": "0x8eb995b11838c69702f5050964990ec89bf8452440c432796e30292330983817",
  "secretPhrase": "day lumber clip wear century elite light rival barely relief tag pledge",
  "secretSeed": "0xcf015e0ddfc47d1897e0c09ea67d9d5edd8d5e8f9d1683032bace30e33ae6bc6",
  "ss58Address": "5FHqnbcu6GkULuqCh8nUBwZeRZhGZ81wynAMnWVPCLyq5QPV"
}


```

2. 然后使用subkey inspect命令查找和sr25519密钥对应的ed25519密钥，后者会被GRANDPA用于最终确认，GRANDPA（GHOST-based Recursive ANcestor Deriving Prefix Agreement，基于GHOST的递归祖先派生前缀协议）是Polkadot中继链的最终决定性工具。
```


```

3.启动Alice和Bob节点时，使用的是内置的预定义链规范（--chain local），下面的命令将其导出到名为customSpec.json的链规范文件中
```
./target/debug/node-template build-spec --disable-default-bootnode --chain local > ./customSpec.json
```

4. 需要关注的customSpec.json文件中的部分是用于生成区块的Aura授权（由palletAura字段表示），以及用于完成区块最终性确认的GRANDPA授权（由palletGrandpa字段表示）

```
../target/debug/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```
