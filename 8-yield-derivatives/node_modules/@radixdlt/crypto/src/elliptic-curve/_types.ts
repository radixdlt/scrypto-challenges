import { ResultAsync } from 'neverthrow'
import { UInt256 } from '@radixdlt/uint256'
import { Hasher } from '../_types'

export type DiffieHellman = (
	publicKeyOfOtherParty: PublicKeyT,
) => ResultAsync<ECPointOnCurveT, Error>

export type Signer = Readonly<{
	sign: (hashedMessage: Buffer) => ResultAsync<SignatureT, Error>

	signUnhashed: (
		input: Readonly<{
			msgToHash: Buffer | string
			hasher?: Hasher
		}>,
	) => ResultAsync<SignatureT, Error>
}>

export type SignatureT = Readonly<{
	r: UInt256
	s: UInt256
	toDER: () => string
	equals: (other: SignatureT) => boolean
}>

// A non-infinity point on the EC curve (e.g. `secp256k1`)
export type ECPointOnCurveT = Readonly<{
	x: UInt256
	y: UInt256
	toBuffer: (includePrefixByte?: boolean) => Buffer
	toString: (includePrefixByte?: boolean) => string
	equals: (other: ECPointOnCurveT) => boolean
	add: (other: ECPointOnCurveT) => ECPointOnCurveT
	multiply: (by: UInt256) => ECPointOnCurveT
	multiplyWithPrivateKey: (privateKey: PrivateKeyT) => ECPointOnCurveT
}>

export type PublicKeyT = Readonly<{
	__hex: string // debug print
	asData: (input: { readonly compressed: boolean }) => Buffer
	toString: (compressed?: boolean) => string
	isValidSignature: (
		input: Readonly<{
			signature: SignatureT
			hashedMessage: Buffer
		}>,
	) => boolean
	decodeToPointOnCurve: () => ECPointOnCurveT
	equals: (other: PublicKeyT) => boolean
}>

export type PrivateKeyT = Signer &
	Readonly<{
		diffieHellman: DiffieHellman
		scalar: UInt256
		publicKey: () => PublicKeyT
		toString: () => string
	}>

export type KeyPairT = Readonly<{
	publicKey: PublicKeyT
	privateKey: PrivateKeyT
}>
