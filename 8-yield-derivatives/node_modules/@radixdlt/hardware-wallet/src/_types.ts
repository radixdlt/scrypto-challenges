import { Observable } from 'rxjs'
import {
	ECPointOnCurveT,
	HDPathRadixT,
	PublicKeyT,
	SignatureT,
} from '@radixdlt/crypto'
import { BuiltTransactionReadyToSign, Network } from '@radixdlt/primitives'

// Semantic versioning, e.g. 1.0.5
export type SemVerT = Readonly<{
	major: number
	minor: number
	patch: number

	equals: (other: SemVerT) => boolean

	// '{major}.{minor}.{patch}'
	toString: () => string
}>

export type AtPath = Readonly<{
	// defaults to: `m/44'/1022'/0'/0/0`
	path?: HDPathRadixT
}>

export type GetPublicKeyInput = AtPath &
	Readonly<{
		display?: boolean
		/// Only relevant if `display` is true, this skips showing BIP32 Path on display.
		verifyAddressOnly?: boolean
	}>

export type SignTXOutput = Readonly<{
	signature: SignatureT
	signatureV: number
	hashCalculatedByLedger: Buffer
}>

export type SignHashInput = GetPublicKeyInput &
	Readonly<{
		hashToSign: Buffer
	}>

export type KeyExchangeInput = AtPath &
	Readonly<{
		publicKeyOfOtherParty: PublicKeyT
		display?: 'encrypt' | 'decrypt'
	}>

export type HardwareSigningKeyT = Readonly<{
	keyExchange: (
		publicKeyOfOtherParty: PublicKeyT,
		display?: 'encrypt' | 'decrypt',
	) => Observable<ECPointOnCurveT>
	publicKey: PublicKeyT

	// Like property `publicKey` but a function and omits BIP32 path on HW display
	getPublicKeyDisplayOnlyAddress: () => Observable<PublicKeyT>

	signHash: (hashedMessage: Buffer) => Observable<SignatureT>
	sign: (
		tx: BuiltTransactionReadyToSign,
		nonXrdHRP?: string,
	) => Observable<SignatureT>
}>

export type SignTransactionInput = Readonly<{
	tx: BuiltTransactionReadyToSign
	path: HDPathRadixT
	nonXrdHRP?: string
}>

export type HardwareWalletT = Readonly<{
	getVersion: () => Observable<SemVerT>
	getPublicKey: (input: GetPublicKeyInput) => Observable<PublicKeyT>
	doSignHash: (input: SignHashInput) => Observable<SignatureT>
	doSignTransaction: (input: SignTransactionInput) => Observable<SignTXOutput>
	doKeyExchange: (input: KeyExchangeInput) => Observable<ECPointOnCurveT>

	makeSigningKey: (
		path: HDPathRadixT,
		verificationPrompt?: boolean,
	) => Observable<HardwareSigningKeyT>
}>
