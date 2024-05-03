import {
	SigningKeyT,
	AccountAddressT,
	isSigningKey,
	isAccountAddress,
} from '@radixdlt/account'
import { AccountT } from './_types'

export const isAccount = (something: unknown): something is AccountT => {
	const inspection = something as AccountT
	return (
		inspection.signingKey !== undefined &&
		isSigningKey(inspection.signingKey) &&
		inspection.address !== undefined &&
		isAccountAddress(inspection.address)
	)
}

const create = (
	input: Readonly<{
		address: AccountAddressT
		signingKey: SigningKeyT
	}>,
): AccountT => {
	const { signingKey, address } = input
	if (!signingKey.publicKey.equals(address.publicKey)) {
		const errMsg = `Incorrect implementation, publicKey of address does not match publicKey of signingKey.`
		console.error(errMsg)
		throw new Error(errMsg)
	}
	const network = address.network
	const publicKey = signingKey.publicKey
	const hdPath = signingKey.hdPath
	return {
		...signingKey, // encrypt, decrypt, sign
		equals: (other: AccountT): boolean => other.publicKey.equals(publicKey),
		signingKey: signingKey,
		type: signingKey.type,
		address,
		network,
		publicKey,
		hdPath,
	}
}

export const Account = {
	create,
}
