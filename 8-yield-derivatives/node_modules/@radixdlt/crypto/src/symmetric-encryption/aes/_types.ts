import { SecureRandom } from '@radixdlt/util'

export type AES_GCM_SealedBoxProps = Readonly<{
	authTag: Buffer
	ciphertext: Buffer
	nonce: Buffer
}>

export type AES_GCM_SealedBoxT = AES_GCM_SealedBoxProps &
	Readonly<{
		combined: () => Buffer
		equals: (other: AES_GCM_SealedBoxT) => boolean
	}>

export type AES_GCM_OPEN_Input = AES_GCM_SealedBoxProps &
	Readonly<{
		symmetricKey: Buffer
		additionalAuthenticationData?: Buffer
	}>

export type AES_GCM_SEAL_Input = Readonly<{
	plaintext: Buffer
	symmetricKey: Buffer
	additionalAuthenticationData?: Buffer
	nonce?: Buffer
	secureRandom?: SecureRandom
}>
