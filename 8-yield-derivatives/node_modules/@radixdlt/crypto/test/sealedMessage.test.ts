import {
	SealedMessage,
	__validateTag,
	__validateNonce,
	AES_GCM_SealedBox,
	PublicKey,
} from '../src'
import { buffersEquals } from '@radixdlt/util'

describe('SealedMessage type', () => {
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

	const input = {
		ephemeralPublicKey,
		nonce,
		authTag,
		ciphertext,
	}

	const nonceAuthCipherHex =
		'dedededededededededededeabababababababababababababababab48656c6c6f20576f726c64'
	const combinedHex = pubKeyHex + nonceAuthCipherHex

	it('can be created', () => {
		const msg = SealedMessage.create(input)._unsafeUnwrap()
		expect(msg.ephemeralPublicKey.equals(ephemeralPublicKey)).toBe(true)
		expect(buffersEquals(msg.nonce, nonce)).toBe(true)
		expect(buffersEquals(msg.authTag, authTag)).toBe(true)
		expect(buffersEquals(msg.ciphertext, ciphertext)).toBe(true)

		const combined = Buffer.concat([
			ephemeralPublicKey.asData({ compressed: true }),
			nonce,
			authTag,
			ciphertext,
		])
		expect(buffersEquals(msg.combined(), combined)).toBe(true)
		expect(msg.combined().toString('hex')).toBe(combinedHex)
	})

	it('can be created from buffer', () => {
		const buffer = Buffer.from(combinedHex, 'hex')
		const msg = SealedMessage.fromBuffer(buffer)._unsafeUnwrap()
		expect(buffersEquals(msg.nonce, nonce)).toBe(true)
		expect(buffersEquals(msg.authTag, authTag)).toBe(true)
		expect(buffersEquals(msg.ciphertext, ciphertext)).toBe(true)

		expect(buffersEquals(msg.combined(), buffer)).toBe(true)
	})

	it('can be created from AES SealedBox', () => {
		const aesBuffer = Buffer.from(nonceAuthCipherHex, 'hex')
		const aesSealedBox = AES_GCM_SealedBox.fromCombinedBuffer(
			aesBuffer,
		)._unsafeUnwrap()

		expect(buffersEquals(aesSealedBox.combined(), aesBuffer)).toBe(true)

		const msg = SealedMessage.fromAESSealedBox(
			aesSealedBox,
			ephemeralPublicKey,
		)._unsafeUnwrap()

		expect(msg.combined().toString('hex')).toBe(combinedHex)
	})

	describe('sealed message length validation functions', () => {
		const makeBuf = (byteCount: number): Buffer =>
			Buffer.from('6a'.repeat(byteCount), 'hex')

		describe('validateNonce', () => {
			it('correct nonce length is valid', done => {
				const buffer = makeBuf(12)
				__validateNonce(buffer).match(
					buf => {
						expect(buffersEquals(buf, buffer)).toBe(true)
						done()
					},
					error => {
						done(
							new Error(
								`Got error, but expected success: ${error}`,
							),
						)
					},
				)
			})

			const nonceErr = (actual: number): string =>
				`Incorrect length of nonce, expected: #12 bytes, but got: #${actual}.`

			it('too short nonce is invalid', done => {
				const shortLength = 11
				const buffer = makeBuf(shortLength)
				__validateNonce(buffer).match(
					_ => {
						done(
							new Error(
								'Buffer passed validation, but we expected a failure.',
							),
						)
					},
					error => {
						expect(error.message).toBe(nonceErr(shortLength))
						done()
					},
				)
			})

			it('too long nonce length is invalid', done => {
				const longLength = 13
				const buffer = makeBuf(longLength)
				__validateNonce(buffer).match(
					_ => {
						done(
							new Error(
								'Buffer passed validation, but we expected a failure.',
							),
						)
					},
					error => {
						expect(error.message).toBe(nonceErr(longLength))
						done()
					},
				)
			})
		})

		describe('validateAuthTag', () => {
			it('correct authTag length is valid', done => {
				const buffer = makeBuf(16)
				__validateTag(buffer).match(
					buf => {
						expect(buffersEquals(buf, buffer)).toBe(true)
						done()
					},
					error => {
						done(
							new Error(
								`Got error, but expected success: ${error}`,
							),
						)
					},
				)
			})

			const tagErrMsg = (actual: number): string =>
				`Incorrect length of auth tag, expected: #16 bytes, but got: #${actual}.`

			it('too short tag length is invalid', done => {
				const shortLength = 15
				const buffer = makeBuf(shortLength)
				__validateTag(buffer).match(
					_ => {
						done(
							new Error(
								'Buffer passed validation, but we expected a failure.',
							),
						)
					},
					error => {
						expect(error.message).toBe(tagErrMsg(shortLength))
						done()
					},
				)
			})

			it('too long tag length is invalid', done => {
				const longLength = 17
				const buffer = makeBuf(longLength)
				__validateTag(buffer).match(
					_ => {
						done(
							new Error(
								'Buffer passed validation, but we expected a failure.',
							),
						)
					},
					error => {
						expect(error.message).toBe(tagErrMsg(longLength))
						done()
					},
				)
			})
		})
	})
})
