/* eslint-disable */
import { err, ok, Result } from 'neverthrow'

export type BufferReaderT = Readonly<{
	finishedParsing: () => boolean
	readNextBuffer: (byteCount: number) => Result<Buffer, Error>
	remainingBytes: () => Buffer
}>

const createBufferReader = (buf: Buffer): BufferReaderT => {
	if (!Buffer.isBuffer(buf)) {
		buf = Buffer.from(buf) // Convert Uint8Array to Buffer for Electron renderer compatibility 💩
	}

	let offset = 0
	let bytesLeftToRead = buf.length

	const readNextBuffer = (byteCount: number): Result<Buffer, Error> => {
		if (byteCount < 0)
			return err(new Error(`'byteCount' must be no negative`))
		if (byteCount === 0) {
			return ok(Buffer.alloc(0))
		}
		if (offset + byteCount > buf.length)
			return err(new Error(`Out of buffer's boundary`))
		const bufToReturn = Buffer.alloc(byteCount)
		buf.copy(bufToReturn, 0, offset, offset + byteCount)

		if (bufToReturn.length !== byteCount) {
			throw new Error(`Incorrect length of newly read buffer...`)
		}

		offset += byteCount
		bytesLeftToRead -= byteCount

		// console.log(`
		// 	🧵🧵🧵
		// 		read: #${byteCount} bytes,
		// 		read buffer: '0x${bufToReturn.toString('hex')}',
		// 		offset: ${offset},
		// 		source buffer: '0x${buf.toString('hex')}',
		// 		length of source buffer: #${buf.length} bytes.
		// 		bytesLeftToRead: #${bytesLeftToRead}
		// 	🧵🧵🧵
		// `)

		return ok(bufToReturn)
	}

	const finishedParsing = (): boolean => {
		if (bytesLeftToRead < 0) {
			throw new Error(`Incorrect implementation, read too many bytes.`)
		}
		return bytesLeftToRead === 0
	}

	return {
		readNextBuffer,
		finishedParsing,
		remainingBytes: (): Buffer => {
			if (finishedParsing()) return Buffer.alloc(0)
			const leftBuf = Buffer.alloc(bytesLeftToRead)
			buf.copy(leftBuf, 0, offset)
			return leftBuf
		},
	}
}
export const BufferReader = {
	create: createBufferReader,
}

export const readBuffer = (
	buffer: Buffer,
): ((byteCount: number) => Result<Buffer, Error>) => {
	const bufferReader: BufferReaderT = createBufferReader(buffer)
	return bufferReader.readNextBuffer
}

/* eslint-enable */
