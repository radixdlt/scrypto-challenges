export type Bech32T = Readonly<{
	hrp: string

	// excluding checksum
	data: Buffer

	// including checksum
	toString: () => string

	equals: (other: Bech32T) => boolean
}>
