# `@radixdlt/account`

Account related APIs for Radix.

## Wallet

We have a `WalletT` type being a **hierchal deterministic wallet** (explained by [Ledger Acadamy](https://www.ledger.com/academy/crypto/what-are-hierarchical-deterministic-hd-wallets) and on [BitcoinWiki](https://en.bitcoinwiki.org/wiki/Deterministic_wallet#HD_Wallet_.E2.80.93_Hierarchical_Deterministic_Wallet)) capable of deriving all "account"s you will need.

The trailing _T_ in `WalletT` is a suffix we use for all `type`s (we don't use [TypeScript `class`es](https://www.typescriptlang.org/docs/handbook/classes.html) at all). We reserve the `Wallet` name as a "namespaces" for our types, providing static-like factory/constructor methods, e.g. `Wallet.create` (N.B. the **lack of** trailing _T_). This decision was taken since we believe you will more often _use the namespace_ `Wallet.create` than you have to _declare the type_ `WalletT`. 

Here follows the generation of a new mnemonic and the creation of a wallet, via the saving of a keystore.

### Simple wallet creation

This outlines the most convenient wallet creation flow using `byEncryptingMnemonicAndSavingKeystore`.

```typescript
import { Mnemonic, Strength, Language } from '@radixdlt/account'

// ⚠️ Require user to backup mnemonic. 
// She will NEVER be able to re-view it.
const mnemonic = Mnemonic.generateNew()

// This will be our "application password" (1️⃣)
// User chooses this, however, please tell her to use a unique, strong randomly generated encryption password. Also urge user to back this up in a safe place. She will need it every time she starts the app.
const keystoreEncryptionPassword = confirmPasswordTextField.value() // or similar

// You need to pass in a function which saves the keystore
// this example uses 'fs' but using a browser you might
// wanna try out https://www.npmjs.com/package/fs-jetpack or similar.
import { PathLike, promises as fsPromises } from 'fs'
const saveKeystoreOnDisk = (keystore: KeystoreT): Promise<void> => {
    const filePath = 'SOME/SUITABLE/PATH/keystore.json'
    const json = JSON.stringify(keystore, null, '\t')
    return fsPromises.writeFile(filePath, json)
}

// `walletResult` has type `ResultAsync<WalletT, Error>`
// `ResultAsync`: github.com/supermacro/neverthrow (2️⃣)
const walletResult = await Wallet.byEncryptingMnemonicAndSavingKeystore({
	mnemonic,
	password: keystoreEncryptionPassword,
	save: saveKeystoreOnDisk,
})

if (walletResult.isErr()) {
	console.log(`🤷‍♂️ Failed to create wallet: ${walletResult.error}`)
} else {
	const wallet = walletResult.value
	// do something with 'wallet'
}
```

1️⃣: The `keystoreEncryptionPassword` will be needed everytime the user re-opens the wallet app after having terminated it. It's used to _decrypt_ the encrypted `hdMasterSeed`. Remember, the keystore is just a JSON file containing an encrypted ciphertext, and metadata about the encryption used to derive said cihpertext. The ciphertext itself is the BIP39 "seed", not the entropy/mnemonic itself.
2️⃣ Read more about [`Result` / `ResultAsync`](https://github.com/supermacro/neverthrow)


### Alternative wallet creation
Alternatively you can use a flow where you have a bit more control. This is basically exactly what `Wallet.byEncryptingMnemonicAndSavingKeystore` above does. 

```typescript
const mnemonic = Mnemonic.generateNew()
// ⚠️ Require user backup mnemonic first!
const masterSeed = HDMasterSeed.fromMnemonic({ mnemonic })

// Tell user to backup encryption password.
const keystoreEncryptionPassword = confirmPasswordTextField.value() // or similar

const walletResult = await Keystore.encryptSecret({
		secret: masterSeed.seed,
		password,
	})
	.map((keystore) => ({ keystore, filePath: keystorePath }))
	.andThen((keystore) => {
		// Save keystore on file and return an `ResultAsync<KeystoreT, Error>
	})
	.map((keystore) => ({ keystore, password: keystoreEncryptionPassword }))
	.andThen(Wallet.fromKeystore)

if (walletResult.isErr()) {
	console.log(`🤷‍♂️ Failed to create wallet: ${walletResult.error}`)
} else {
	const wallet = walletResult.value
	// do something with 'wallet'
}
```

### Open wallet (app start)

```typescript
// Path to where location where the keystore.json file will be saved.
import { Keystore } from "./keystore";
import { PathLike, promises as fsPromises } from 'fs'

// Each time GUI wallet starts ask user for encryption password in GUI
const keystoreEncryptionPassword = passwordTextField.value() // or similar

const loadKeystoreOnDisk = (): Promise<KeystoreT> => {
	const filePath = 'SOME/SUITABLE/PATH/keystore.json'
	return fsPromises.readFile(filePath)
         .then(buffer => Keystore.fromBuffer(buffer))
}

const walletResult = await Wallet.byLoadingAndDecryptingKeystore({
	password: keystoreEncryptionPassword,
	load: loadKeystoreOnDisk
})

if (walletResult.isErr()) {
	console.log(`🤷‍♂️ Failed to create wallet: ${walletResult.error}`)
} else {
	const wallet = walletResult.value
	// do something with 'wallet'
}
```
