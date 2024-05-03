export type Int32 = number

export type BIP32T = Readonly<{
	pathComponents: BIP32PathComponentT[]
	toString: () => string
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	equals: (other: any) => boolean
}>

export type BIP32PathSimpleT = Readonly<{
	index: Int32
	isHardened: boolean
}>

export type BIP32PathComponentT = BIP32PathSimpleT &
	Readonly<{
		toString: () => string

		// Not to be confused with the 'index', this is the position of this path component
		// inside a BIP32 path, e.g. `5/3/1` the component '5' has level 0 and '1' has level 2.
		level: number

		// E.g. 'purpose', 'coinType' 'account', 'change', 'address_index'
		name?: string

		// For `0'` the value 0 is returned, even though it is hardened.
		value: () => Int32
	}>
