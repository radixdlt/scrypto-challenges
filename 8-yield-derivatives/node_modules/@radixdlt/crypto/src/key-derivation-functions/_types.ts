export type ScryptParamsT = Readonly<{
	// "N", CPU/memory cost parameter, must be power of 2.
	costParameterN: number
	costParameterC: number

	// "r", blocksize
	blockSize: number

	// "p"
	parallelizationParameter: number

	// "dklen"
	lengthOfDerivedKey: number

	salt: string
}>
