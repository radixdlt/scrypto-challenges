import { Result, err, ok } from 'neverthrow'
import blake from 'blakejs'
import { Buffer } from 'buffer'

const toArrayBuffer = (buffer: Buffer): ArrayBuffer => {
  const arrayBuffer = new ArrayBuffer(buffer.length)
  const view = new Uint8Array(arrayBuffer)
  for (let i = 0; i < buffer.length; ++i) {
    view[i] = buffer[i]
  }
  return arrayBuffer
}

export const bufferToUnit8Array = (buffer: Buffer): Uint8Array =>
  new Uint8Array(toArrayBuffer(buffer))

export const blake2b = (input: Buffer): Result<Buffer, Error> => {
  try {
    return ok(blake.blake2bHex(bufferToUnit8Array(input), undefined, 32)).map(
      (hex) => Buffer.from(hex, 'hex')
    )
  } catch (error) {
    return err(error as Error)
  }
}
