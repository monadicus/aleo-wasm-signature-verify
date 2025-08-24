## Aleo Wasm Verify

This is a simple wasm library based on the utilities provided by [aleo-testnet2-tools](https://github.com/monadicus/aleo-testnet2-tools). The output is built web and nodeJS.

This package contains a few simple functions for verifying aleo Aleo addresses and signatures:

|Signature|Returns|Description|
|-|-|-|
|`verify_signature(address, message, signature)`|`boolean`|Returns true for valid mainnet signatures. Throws errors for parsing errors.|
|`verify_address(address)`|`boolean`|Returns true for valid testnet2 addresses.|

### Usage

```js
import initWasm, { verify_address, verify_signature } from 'aleo-wasm-verify';
// or dynamic import
const { default: initWasm, verify_address, verify_signature } = await import('aleo-wasm-verify');

// init the wasm module
await wasm();

// use the functions

const addrOk = verify_address(address);
const sigOk = verify_signature(address, message, signature);
```

### Compiling from source

1. Install [wasm-pack](https://drager.github.io/wasm-pack/installer/)
1. `wasm-pack build --release --target web`
