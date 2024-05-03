import { Byte, SecureRandom } from '@radixdlt/util'
import { ResultAsync } from 'neverthrow'
import { ECPointOnCurveT, PublicKeyT } from '../elliptic-curve'

export type MessageEncryptionInput = Readonly<{
	plaintext: Buffer | string
	diffieHellmanPoint: () => ResultAsync<ECPointOnCurveT, Error>
	secureRandom?: SecureRandom
}>

export type MessageDecryptionInput = Readonly<{
	encryptedMessage: Buffer | EncryptedMessageT
	diffieHellmanPoint: () => ResultAsync<ECPointOnCurveT, Error>
}>

export const ENCRYPTION_SCHEME_BYTES = 1

export const MESSAGE_TYPE_BYTES = 1

export enum MessageType {
	PLAINTEXT = 0x00,
	ENCRYPTED = 0x01,
	HEX = 0x1e,
}

export enum EncryptionScheme {
	NONE = 0x00,
	DH_ADD_EPH_AESGCM256_SCRYPT_000 = 0xff,
}

export type SealedMessageT = Readonly<{
	/* The public key of the ephemeral key pair. 33 bytes */
	ephemeralPublicKey: PublicKeyT

	/* The nonce used to encrypt the data. 12 bytes. AKA "IV". */
	nonce: Buffer

	/* An authentication tag. 16 bytes, e.g. AES GCM tag. */
	authTag: Buffer

	/* The encrypted data. Max 162 bytes. */
	ciphertext: Buffer

	combined: () => Buffer
}>

type Message<Kind extends keyof typeof MessageType> = {
	kind: Kind
}

// Max 255 bytes
export type EncryptedMessageT = Message<'ENCRYPTED'> & {
	encryptionScheme: EncryptionScheme

	/* Encrypted message with metadata containing about how it can be decrypted. Max 223 bytes. */
	sealedMessage: SealedMessageT

	combined: () => Buffer
}

export type PlaintextMessageT = Message<'PLAINTEXT'> & {
	plaintext: string
	bytes: Buffer
}
