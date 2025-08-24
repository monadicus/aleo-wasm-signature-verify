## Aleo Wasm Verify

This is a simple wasm library based on the utilities provided by [aleo-testnet2-tools](https://github.com/monadicus/aleo-testnet2-tools). The output is built web and nodeJS.

This package contains a few simple functions for verifying aleo Aleo addresses and signatures:

|Signature|Returns|Description|
|-|-|-|
|`verify_signature(address, message, signature)`|`boolean`|Returns true for valid mainnet signatures. Throws errors for parsing errors.|
|`verify_address(address)`|`boolean`|Returns true for valid testnet2 addresses.|

### Compiling from source

1. Install [wasm-pack](https://drager.github.io/wasm-pack/installer/)
1. `wasm-pack build --release --target web`
