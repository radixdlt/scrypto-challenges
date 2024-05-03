import { SecureRandom } from './_types'
import { Result, ok, err } from 'neverthrow'

/* eslint-disable */

/**
 * randomBytes
 *
 * Uses JS-native CSPRNG to generate a specified number of bytes.
 *
 * @param {number} byteCount number of bytes to generate
 * @returns {Result<Buffer, Error>} result of random byte generation.
 */
const randomBytes = (byteCount: number): Result<string, Error> => {
	let buffer: Buffer = Buffer.alloc(byteCount)
	if (
		typeof window !== 'undefined' &&
		window.crypto &&
		window.crypto.getRandomValues
	) {
		const bytes = window.crypto.getRandomValues(new Uint8Array(byteCount))
		buffer = Buffer.from(bytes)
	} else if (typeof require !== 'undefined') {
		const sodium = require('sodium-native')
		sodium.randombytes_buf(buffer)
	} else {
		return err(new Error('Unable to generate safe random numbers.'))
	}

	const byteArray = new Uint8Array(
		buffer.buffer,
		buffer.byteOffset,
		buffer.byteLength / Uint8Array.BYTES_PER_ELEMENT,
	)
	let byteString = ''
	for (let i = 0; i < byteCount; i++) {
		byteString += ('00' + byteArray[i].toString(16)).slice(-2)
	}

	return ok(byteString)
}

export const secureRandomGenerator: SecureRandom = {
	randomSecureBytes: (byteCount: number) =>
		randomBytes(byteCount)._unsafeUnwrap(),
}
