import { SecureRandom, secureRandomGenerator } from '@radixdlt/util'
import { PrivateKey } from './privateKey'
import { KeyPairT, PrivateKeyT } from './_types'

const fromPrivateKey = (privateKey: PrivateKeyT): KeyPairT => ({
	privateKey,
	publicKey: privateKey.publicKey(),
})

const generateNew = (
	secureRandom: SecureRandom = secureRandomGenerator,
): KeyPairT => {
	const privateKey = PrivateKey.generateNew(secureRandom)
	return fromPrivateKey(privateKey)
}

export const KeyPair = {
	generateNew,
	fromPrivateKey,
}
