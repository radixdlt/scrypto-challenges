import crypto from 'node:crypto'

export const secureRandom = (byteCount: number): string =>
  crypto.randomBytes(byteCount).toString('hex')
