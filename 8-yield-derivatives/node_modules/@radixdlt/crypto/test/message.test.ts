import {
	__validateEncryptedMessageLength,
	Message,
} from '../src/encryption/message'
import { buffersEquals } from '@radixdlt/util'
import {
	EncryptedMessageT,
	EncryptionScheme,
	MessageType,
	PlaintextMessageT,
	PublicKey,
	SealedMessage,
} from '../src'

const bufWByteCount = (byteCount: number, chars: string): Buffer =>
	Buffer.from(chars.repeat(byteCount), 'hex')

const pubKeyHex =
	'0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'
const ephemeralPublicKey = PublicKey.fromBuffer(
	Buffer.from(pubKeyHex, 'hex'),
)._unsafeUnwrap()

const plaintext = 'Hello World'
const ciphertext = Buffer.from(plaintext, 'utf8')
const nonce = bufWByteCount(SealedMessage.nonceByteCount, 'de')
const authTag = bufWByteCount(SealedMessage.authTagByteCount, 'ab')

const sealedMessageInput = {
	ephemeralPublicKey,
	nonce,
	authTag,
	ciphertext,
}

const sealedMessage = SealedMessage.create(sealedMessageInput)._unsafeUnwrap()

describe('EncryptedMessage', () => {
	const makeBuf = (byteCount: number): Buffer =>
		Buffer.from('6a'.repeat(byteCount), 'hex')

	describe('validateEncryptedMessageLength', () => {
		it('correct msg length is valid', done => {
			const buffer = makeBuf(100)
			__validateEncryptedMessageLength(buffer).match(
				buf => {
					expect(buffersEquals(buf, buffer)).toBe(true)
					done()
				},
				error => {
					done(new Error(`Got error, but expected success: ${error}`))
				},
			)
		})

		it('too short msg length is invalid', done => {
			const shortLength = 10
			const buffer = makeBuf(shortLength)
			__validateEncryptedMessageLength(buffer).match(
				_ => {
					done(
						new Error(
							'Buffer passed validation, but we expected a failure.',
						),
					)
				},
				error => {
					expect(error.message).toBe(
						`Incorrect length of encryptedMessage, expected min: #${Message.minLengthEncryptedMessage} bytes, but got: #${shortLength}.`,
					)
					done()
				},
			)
		})

		it('too long msg is invalid', done => {
			const longLength = 256
			const buffer = makeBuf(longLength)
			__validateEncryptedMessageLength(buffer).match(
				_ => {
					done(
						new Error(
							'Buffer passed validation, but we expected a failure.',
						),
					)
				},
				error => {
					expect(error.message).toBe(
						`Incorrect length of encryptedMessage, expected max: #255 bytes, but got: #${longLength}.`,
					)
					done()
				},
			)
		})

		it('should create a plaintext message', () => {
			const plaintext = 'Example message'
			const message = Message.createPlaintext(plaintext)

			expect(message.plaintext).toEqual(plaintext)
			expect(message.kind).toEqual('PLAINTEXT')
			expect(message.plaintext).toMatch(plaintext)
		})

		it('should create an encrypted message', () => {
			const message = Message.createEncrypted(
				EncryptionScheme.DH_ADD_EPH_AESGCM256_SCRYPT_000,
				sealedMessage,
			)._unsafeUnwrap()

			expect(message.kind).toEqual('ENCRYPTED')
			expect(message.encryptionScheme).toEqual(
				EncryptionScheme.DH_ADD_EPH_AESGCM256_SCRYPT_000,
			)
			expect(message.sealedMessage.combined()).toEqual(
				sealedMessage.combined(),
			)
		})

		it('should create a plaintext message from buffer', () => {
			const messageString = 'message'
			const message = Buffer.from(messageString)
			const payload = Buffer.alloc(61, 0)
			payload.fill(message, 0, message.length)
			const messageBytes = Buffer.concat([
				Buffer.from([MessageType.PLAINTEXT, EncryptionScheme.NONE]),
				payload,
			])
			const plaintextMsg = Message.fromBuffer(
				messageBytes,
			)._unsafeUnwrap()

			expect(plaintextMsg.kind).toEqual('PLAINTEXT')
			expect((plaintextMsg as PlaintextMessageT).plaintext).toMatch(
				messageString,
			)
			expect((plaintextMsg as PlaintextMessageT).bytes).toEqual(
				messageBytes,
			)
		})

		it('should create an encrypted message from buffer', () => {
			const messageBytes = Buffer.concat([
				Buffer.from([
					MessageType.ENCRYPTED,
					EncryptionScheme.DH_ADD_EPH_AESGCM256_SCRYPT_000,
				]),
				sealedMessage.combined(),
			])
			const encryptedMsg = Message.fromBuffer(
				messageBytes,
			)._unsafeUnwrap()

			expect(encryptedMsg.kind).toEqual('ENCRYPTED')
			expect(
				(encryptedMsg as EncryptedMessageT).encryptionScheme,
			).toEqual(EncryptionScheme.DH_ADD_EPH_AESGCM256_SCRYPT_000)
			expect(
				(encryptedMsg as EncryptedMessageT).sealedMessage.combined(),
			).toEqual(sealedMessage.combined())
			expect((encryptedMsg as EncryptedMessageT).combined()).toEqual(
				messageBytes,
			)
		})

		it('should fail to create from buffer with an invalid message type', () => {
			const messageString = 'message'
			const message = Buffer.from(messageString)
			const payload = Buffer.alloc(61, 0)
			payload.fill(message, 0, message.length)
			const invalidType = 255
			const messageBytes = Buffer.concat([
				Buffer.from([invalidType, EncryptionScheme.NONE]),
				payload,
			])
			const plaintextMsgError = Message.fromBuffer(
				messageBytes,
			)._unsafeUnwrapErr()

			expect(plaintextMsgError.message).toMatch(
				`Unknown message type: ${invalidType}`,
			)
		})

		it('should fail to create from buffer with an invalid encryption scheme', () => {
			const invalidScheme = 1
			const messageBytes = Buffer.concat([
				Buffer.from([MessageType.ENCRYPTED, invalidScheme]),
				sealedMessage.combined(),
			])
			const encryptedMsgError = Message.fromBuffer(
				messageBytes,
			)._unsafeUnwrapErr()

			expect(encryptedMsgError.message).toMatch(
				`Unknown encryption scheme: ${invalidScheme}`,
			)
		})

		it('should fail to create from buffer with an invalid combination of message type and encryption scheme', () => {
			const messageBytes = Buffer.concat([
				Buffer.from([MessageType.ENCRYPTED, EncryptionScheme.NONE]),
				sealedMessage.combined(),
			])
			const encryptedMsgError = Message.fromBuffer(
				messageBytes,
			)._unsafeUnwrapErr()

			expect(encryptedMsgError.message).toMatch(
				`Invalid combination of message type ${MessageType.ENCRYPTED} and encryption scheme ${EncryptionScheme.NONE}.`,
			)
		})
	})
})
