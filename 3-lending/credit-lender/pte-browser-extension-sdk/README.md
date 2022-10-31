
## PTE Browser extension SDK

### Example

```typescript
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

const accountAddress = await getAccountAddress();
console.log("Account address: " + accountAddress);

const componentAddress = '0276b537d9fc474d86edd48bfaa2843e87b48765767357ab9e403d';
const manifest = new ManifestBuilder()
  .withdrawFromAccountByAmount(accountAddress, 1, '030000000000000000000000000000000000000000000000000004')
  .callMethod(componentAddress, 'buy_gumball', ['Decimal("1.0")'])
  .callMethodWithAllResources(accountAddress, 'deposit_batch')
  .build()
  .toString();
console.log("Manifest: " + manifest);

const receipt = await signTransaction(manifest);
console.log("Receipt: "  + receipt);
```

### Build

```
npm install
npm run build
```

### Test

```
npm run test
```
