// eslint-disable-next-line max-params
export const buffersEquals = (lhs: Buffer, rhs: Buffer): boolean =>
	Buffer.compare(lhs, rhs) === 0
