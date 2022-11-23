
## PTE Manifest Compiler

### Example Use

```typescript
import init, { compile_with_nonce } from "pte-manifest-compiler";

await init();
const transaction = compile_with_nonce(txManifest, BigInt(1));
console.log(transaction);
```

### Build

```
wasm-pack build --target web
```

### Test in Headless Browsers

```
wasm-pack test --headless --firefox
```

### Publish to NPM

```
wasm-pack publish --target web
```