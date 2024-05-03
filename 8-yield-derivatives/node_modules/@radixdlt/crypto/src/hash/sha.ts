import { sha256 as SHA256 } from 'hash.js'
import { Hasher } from '../_types'

const toBuffer = (input: Buffer | string): Buffer =>
	typeof input === 'string' ? Buffer.from(input, 'utf-8') : input

export const sha256: Hasher = (input: Buffer | string): Buffer =>
	Buffer.from(SHA256().update(toBuffer(input)).digest())

export const sha256Twice: Hasher = (input: Buffer | string): Buffer =>
	sha256(sha256(toBuffer(input)))
