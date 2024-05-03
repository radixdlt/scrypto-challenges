# uint256
Unsigned 256 bit Integer type

[![npm](https://img.shields.io/npm/v/uint256.svg)](https://www.npmjs.com/package/uint256)

## Install
`npm install uint256`

## Usage
```typescript
import { UInt256, U256 } from 'uint256';

const now = Date.now();
const scale = 18;
const digits = U256(10).pow(scale);

const POUND = (number: number) => U256(number).mul(digits);
const toPound = (number: UInt256, fraction = false) => {
  if (!fraction) {
    return String(number.div(digits));
  }
  const str = String(number);
  if (scale < 1) {
    return str;
  }
  return `${str.substring(0, str.length - scale)}.${str.substr(-scale)}`;
};

const a = POUND(11000);
const b = POUND(20000);

console.log('scale', scale);
console.log('a', toPound(a));
console.log('b', toPound(b));
console.log('mul', toPound(a.mul(b).div(digits)));
console.log('add', toPound(a.add(b)));
console.log('div', toPound(b.div(a.div(digits)), true));
console.log('sub', toPound(b.sub(a)));
console.log('elapsed(ms)', Date.now() - now);

// scale 18
// a 11000
// b 20000
// mul 220000000
// add 31000
// div 1.818181818181818181
// sub 9000
// elapsed(ms) 8
```