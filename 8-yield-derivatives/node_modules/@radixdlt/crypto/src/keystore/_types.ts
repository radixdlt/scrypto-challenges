import { ScryptParamsT } from '../key-derivation-functions'

export type KeystoreCryptoCipherParamsT = Readonly<{
	nonce: string
}>

export type KeystoreCryptoT = Readonly<{
	cipher: string
	cipherparams: KeystoreCryptoCipherParamsT
	ciphertext: string
	kdf: string
	kdfparams: ScryptParamsT
	mac: string
}>

export type KeystoreT = Readonly<{
	memo?: string
	crypto: KeystoreCryptoT
	id: string
	version: number
}>
