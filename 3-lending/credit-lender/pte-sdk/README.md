
## PTE SDK

This is a Typescript library for constructing manifest and interacting with Babylon PTE service.

### Example Use

```typescript
import { ManifestBuilder, DefaultApi } from "pte-sdk";

const manifest = new ManifestBuilder()
    .callMethod('02e97040eb89efb88acc32577cecd380319c14777ea2aa98cddfad', 'withdraw_by_amount', [
        'Decimal("1")',
        'ResourceAddress("030000000000000000000000000000000000000000000000000004")'
    ])
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd_bucket')
    .callMethod('0290b7cbb58c5b43cf0e99534e8161bd7ce561c090adc0fdacac88', 'buy_gumball', [
        'Bucket("xrd_bucket")'
    ])
    .callMethodWithAllResources('02e97040eb89efb88acc32577cecd380319c14777ea2aa98cddfad', 'deposit_batch')
    .build();

const api = new DefaultApi();
const receipt = await api.submitTransaction({
    transaction: {
        manifest: manifest.toString(),
        nonce: {
            value: 4083048708
        },
        signatures: [
            {
                publicKey: '04704ab82d9f5791ba46a134e5c1f08bf131adf53ab7e9c39eb3784ab35be7880432af5bbd67232d4d15b40a6cd676aa69568b95e754c79274648d80b95c4185b0',
                signature: '051dc7a42a3ba8ebd3608913c1dfd424601acaa3a5ef33de6288233d2fedb7c33849aecc8ef2c50ae75af5ea9cec1ecd6f04d2ddd0b07f6c79078c161ee56335'
            }
        ]
    }
})

console.log(receipt);
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