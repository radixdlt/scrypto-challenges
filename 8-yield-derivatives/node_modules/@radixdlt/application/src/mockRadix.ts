import {
	AmountOrUnsafeInput,
	AmountT,
	uint256Max,
	Network,
} from '@radixdlt/primitives'
import {
	AccountAddress,
	AccountAddressT,
	ResourceIdentifier,
	ResourceIdentifierT,
	ValidatorAddress,
	ValidatorAddressT,
} from '@radixdlt/account'
import { Observable, of } from 'rxjs'
import {
	BuiltTransaction,
	ExecutedTransaction,
	FinalizedTransaction,
	NetworkTransactionDemand,
	NetworkTransactionThroughput,
	PendingTransaction,
	SignedTransaction,
	SimpleExecutedTransaction,
	SimpleTokenBalance,
	SimpleTokenBalances,
	StakePosition,
	StakePositions,
	StatusOfTransaction,
	Token,
	TransactionHistory,
	TransactionHistoryRequestInput,
	TransactionIdentifierT,
	TransactionIntent,
	TransactionStatus,
	TransactionType,
	UnstakePosition,
	UnstakePositions,
	Validator,
	Validators,
	ValidatorsRequestInput,
	TransactionIdentifier,
} from './dto'
import { RadixCoreAPI } from './api'
import { shareReplay } from 'rxjs/operators'
import { PrivateKey, PublicKeyT, sha256 } from '@radixdlt/crypto'
import { ActionType, ExecutedAction } from './actions'
import { Amount } from '@radixdlt/primitives'

export const xrd: Token = {
	name: 'Rad',
	rri: ResourceIdentifier.fromUnsafe('xrd_tr1qyf0x76s')._unsafeUnwrap(),
	symbol: 'XRD',
	description: 'The native coin of Radix network',
	granularity: Amount.fromUnsafe(1)._unsafeUnwrap(),
	isSupplyMutable: false,
	currentSupply: uint256Max,
	tokenInfoURL: new URL('https://www.radixdlt.com'),
	iconURL: new URL('https://www.image.radixdlt.com/'),
}

export const fooToken: Token = {
	name: 'Foo token',
	rri: ResourceIdentifier.fromUnsafe('xrd_tr1qyf0x76s')._unsafeUnwrap(),
	symbol: 'FOO',
	description: 'FOOest token.',
	granularity: Amount.fromUnsafe(1)._unsafeUnwrap(),
	isSupplyMutable: false,
	currentSupply: uint256Max,
	tokenInfoURL: new URL('https://www.footoken.com'),
	iconURL: new URL('https://www.image.footoken.com/'),
}

export const barToken: Token = {
	name: 'Bar token',
	rri: ResourceIdentifier.fromUnsafe('xrd_tr1qyf0x76s')._unsafeUnwrap(),
	symbol: 'BAR',
	description: 'Bar token. Granularity E-3.',
	granularity: Amount.fromUnsafe(1000)._unsafeUnwrap(),
	isSupplyMutable: true,
	currentSupply: uint256Max,
	tokenInfoURL: new URL('https://www.bartoken.com'),
	iconURL: new URL('https://www.image.bartoken.com/'),
}

export const goldToken: Token = {
	name: 'Gold token',
	rri: ResourceIdentifier.fromUnsafe('xrd_tr1qyf0x76s')._unsafeUnwrap(),
	symbol: 'GOLD',
	description: 'Gold token. Granularity E-12.',
	granularity: Amount.fromUnsafe(1_000_000)._unsafeUnwrap(),
	isSupplyMutable: false,
	currentSupply: uint256Max,
	tokenInfoURL: new URL('https://www.goldtoken.com'),
	iconURL: new URL('https://www.image.goldtoken.com/'),
}

export const radixWrappedBitcoinToken: Token = {
	name: 'Bitcoin (wrapped on Radix)',
	rri: ResourceIdentifier.fromUnsafe('xrd_tr1qyf0x76s')._unsafeUnwrap(),
	symbol: 'BTCRW',
	description: 'Radix wrapped Bitcoin. Granularity E-18.',
	granularity: Amount.fromUnsafe(1)._unsafeUnwrap(),
	isSupplyMutable: true,
	currentSupply: uint256Max,
	tokenInfoURL: new URL('https://www.bitcoin.radix.com'),
	iconURL: new URL('https://www.image.bitcoin.radix.com/'),
}

export const radixWrappedEtherToken: Token = {
	name: 'Ether (wrapped on Radix)',
	rri: ResourceIdentifier.fromUnsafe('xrd_tr1qyf0x76s')._unsafeUnwrap(),
	symbol: 'ETHRW',
	description: 'Radix wrapped Ether. Granularity E-9.',
	granularity: Amount.fromUnsafe(1_000_000_000)._unsafeUnwrap(),
	isSupplyMutable: true,
	currentSupply: uint256Max,
	tokenInfoURL: new URL('https://www.ether.radix.com'),
	iconURL: new URL('https://www.image.ether.radix.com/'),
}

export const __fallBackAlexToken: Token = {
	name: 'Alex token',
	rri: ResourceIdentifier.fromUnsafe('xrd_tr1qyf0x76s')._unsafeUnwrap(),
	symbol: 'ALEX',
	description:
		'Fallback token for when token for requested symbol was not found.',
	granularity: Amount.fromUnsafe(1)._unsafeUnwrap(),
	isSupplyMutable: true,
	currentSupply: uint256Max,
	tokenInfoURL: new URL('https://www.alex.token.com'),
	iconURL: new URL('https://www.image.alex.token.com/'),
}

export const balanceOfFor = (
	input: Readonly<{
		token: Token
		amount: AmountOrUnsafeInput
	}>,
): SimpleTokenBalance => {
	const amt = Amount.fromUnsafe(input.amount)._unsafeUnwrap()

	return {
		tokenIdentifier: input.token.rri,
		amount: amt.lt(input.token.currentSupply)
			? amt
			: input.token.currentSupply,
	}
}

export const balancesFor = (
	address: AccountAddressT,
	amount: number,
): SimpleTokenBalances => ({
	owner: address,
	tokenBalances: [
		balanceOfFor({
			token: xrd,
			amount,
		}),
	],
})

const differentTokens: Token[] = [
	xrd,
	fooToken,
	barToken,
	radixWrappedBitcoinToken,
	radixWrappedEtherToken,
	goldToken,
]

// PLEASE KEEP - used as Cast of characters: https://en.wikipedia.org/wiki/Alice_and_Bob#Cast_of_characters

export const tokenByRRIMap: Map<
	ResourceIdentifierT,
	Token
> = differentTokens.reduce(
	(a: Map<ResourceIdentifierT, Token>, b: Token) => a.set(b.rri, b),
	new Map<ResourceIdentifierT, Token>(),
)

const detPRNGWithBuffer = (buffer: Buffer): (() => number) => {
	const bufCopy = Buffer.from(buffer)
	let bytes = Buffer.from(buffer)
	return (): number => {
		if (bytes.length === 0) {
			bytes = sha256(bufCopy)
		}
		const lengthToSlice = 2
		const buf = bytes.slice(0, lengthToSlice)
		bytes = bytes.slice(lengthToSlice, bytes.length)
		return Number.parseInt(buf.toString('hex'), 16)
	}
}

const addressesString: string[] = [
	'tdx1qspksqs77z9e24e2dr9t5de6a9kymzhszp9k7jmr2ldkzl4hvn45xsqk409dt',
	'tdx1qspksqs77z9e24e2dr9t5de6a9kymzhszp9k7jmr2ldkzl4hvn45xsqk409dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
]

const characterNames: string[] = [
	'alice',
	'bob',
	'carol',
	'dan',
	'erin',
	'frank',
	'grace',
	'heidi',
	'ivan',
	'judy',
	'klara',
	'leonard',
	'mallory',
	'niaj',
	'olivia',
	'peggy',
	'quentin',
	'rupert',
	'stella',
	'ted',
	'ursula',
	'victor',
	'webdy',
	'xerxez',
	'yara',
	'zelda',
]
/*
* [Property in keyof ReturnType<typeof getAPI>]: ReturnType<
		typeof getAPI
	>[Property]
* */
export const castOfCharacters: AccountAddressT[] = addressesString
	.map(s =>
		AccountAddress.fromUnsafe(s)._unsafeUnwrap({ withStackTrace: true }),
	)
	.slice(0, characterNames.length)
export const alice = castOfCharacters[0]
export const bob = castOfCharacters[1]
export const carol = castOfCharacters[2]
export const dan = castOfCharacters[3]
export const erin = castOfCharacters[4]

const makeListOfValidatorAddresses = (): ValidatorAddressT[] => {
	const stringAddresses = [
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
		'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
	]

	return stringAddresses.map(s =>
		ValidatorAddress.fromUnsafe(s)._unsafeUnwrap({ withStackTrace: true }),
	)
}

const listOfValidatorAddresses: ValidatorAddressT[] = makeListOfValidatorAddresses()

const detRandomValidatorAddressWithPRNG = (
	anInt: () => number,
) => (): ValidatorAddressT => {
	const randomInt = anInt()
	const index = randomInt % (listOfValidatorAddresses.length - 1)
	return listOfValidatorAddresses[index]
}

const randomValidatorList = (
	size: number,
	validatorAddress?: ValidatorAddressT,
): Validator[] => {
	const validatorList: Validator[] = []
	const randomBuf =
		validatorAddress !== undefined
			? sha256(validatorAddress.toString())
			: sha256(size.toString(16))
	const prng = detPRNGWithBuffer(randomBuf)

	const detRandomValidatorAddress = detRandomValidatorAddressWithPRNG(prng)

	const listSize = prng() % 5 === 1 ? size - Math.round(size / 2) : size

	for (let i = 0; i < listSize; i++) {
		const random = prng()
		const ownerAddress = castOfCharacters[random % castOfCharacters.length]
		const name = characterNames[random % characterNames.length]
		const amount = Amount.fromUnsafe(random)._unsafeUnwrap()
		const bool = random % 2 === 0

		validatorList.push({
			address: detRandomValidatorAddress(),
			ownerAddress,
			name,
			infoURL: new URL('https://rewards.radixtokens.comcom'),
			totalDelegatedStake: amount,
			ownerDelegation: amount,
			validatorFee: 2.5,
			registered: bool,
			isExternalStakeAccepted: bool,
			uptimePercentage: 100.0,
			proposalsMissed: 10,
			proposalsCompleted: 20,
		})
	}
	return validatorList
}

const randomUnsignedTransaction = (
	transactionIntent: TransactionIntent,
): BuiltTransaction => {
	const transactionIntentDet = {
		...transactionIntent,
		actions: transactionIntent.actions.map(a => ({
			...a,
		})),
	}

	const detBlob = JSON.stringify(transactionIntentDet, null, 4)
	const blobBytes = Buffer.from(detBlob)
	const bytes32 = sha256(blobBytes)

	const anInt = detPRNGWithBuffer(bytes32)

	return {
		transaction: {
			blob: blobBytes.toString('hex'),
			hashOfBlobToSign: bytes32.toString('hex'),
		},
		fee: Amount.fromUnsafe(anInt())._unsafeUnwrap(),
	}
}

const randomPendingTransaction = (signedTx: SignedTransaction) => ({
	txID: TransactionIdentifier.create(
		sha256(Buffer.from(signedTx.transaction.blob)),
	)._unsafeUnwrap(),
	blob: 'awd',
})

const detRandomSignedUnconfirmedTransaction = (
	signedTransaction: SignedTransaction,
): FinalizedTransaction => ({
	...randomPendingTransaction(signedTransaction),
})

const rndDemand = detPRNGWithBuffer(Buffer.from('dmnd'))
const randomDemand = (): NetworkTransactionDemand => ({
	tps: rndDemand() % 200,
})

const rndThroughput = detPRNGWithBuffer(Buffer.from('trpt'))
const randomThroughput = (): NetworkTransactionDemand => ({
	tps: rndThroughput() % 200,
})

const detPRNGWithPubKey = (pubKey: PublicKeyT): (() => number) => {
	// cannot use first, since it is always 02 or 03
	const bytes = pubKey.asData({ compressed: true }).slice(1, 33)
	return detPRNGWithBuffer(bytes)
}

type BalanceOfTokenWithInfo = Readonly<{
	token: Token
	amount: AmountT
}>

const detRandBalanceOfTokenWithInfo = (
	png: () => number,
): BalanceOfTokenWithInfo[] => {
	const anInt = png
	const availableTokens = [...differentTokens]

	const deterministicRandomToken = (): Token => {
		const tokenCount = availableTokens.length
		const tokenIndex = anInt() % tokenCount
		const token = availableTokens[tokenIndex]
		availableTokens.splice(tokenIndex, 1)
		return token
	}

	const size = Math.max(anInt() % availableTokens.length, 1)

	return Array(size)
		.fill(undefined)
		.map(
			(_): BalanceOfTokenWithInfo => {
				const token = deterministicRandomToken()
				const amtOrZero = anInt() % 10_000
				const amtFactor = Amount.fromUnsafe(
					Math.max(10, amtOrZero),
				)._unsafeUnwrap()

				const amount = Amount.fromUnsafe(
					token.granularity.mul(amtFactor),
				)._unsafeUnwrap()
				return {
					token,
					amount,
				}
			},
		)
}

export const deterministicRandomBalancesForAddress = (
	address: AccountAddressT,
): SimpleTokenBalances => {
	const anInt = detPRNGWithPubKey(address.publicKey)

	const tokenBalances = detRandBalanceOfTokenWithInfo(anInt).map(bti =>
		balanceOfFor(bti),
	)

	return {
		owner: address,
		tokenBalances,
	}
}

export const deterministicRandomUnstakesForAddress = (
	address: AccountAddressT,
): UnstakePositions => {
	const anInt = detPRNGWithPubKey(address.publicKey)
	const size = anInt() % 7
	return Array(size)
		.fill(undefined)
		.map(
			(_, index): UnstakePosition => {
				const detRandomValidatorAddress = detRandomValidatorAddressWithPRNG(
					anInt,
				)

				const validator: ValidatorAddressT = detRandomValidatorAddress()
				const amount = Amount.fromUnsafe(anInt())._unsafeUnwrap()

				const bytesFromIndex = Buffer.allocUnsafe(2)
				bytesFromIndex.writeUInt16BE(index)
				const txIDBuffer = sha256(
					Buffer.concat([
						address.publicKey.asData({ compressed: true }),
						bytesFromIndex,
					]),
				)

				const withdrawTxID = TransactionIdentifier.create(
					txIDBuffer,
				)._unsafeUnwrap()

				const epochsUntil = anInt() % 5
				return {
					amount,
					validator,
					epochsUntil: epochsUntil > 60 ? 0 : epochsUntil,
					withdrawTxID,
				}
			},
		)
}

export const deterministicRandomStakesForAddress = (
	address: AccountAddressT,
): StakePositions =>
	deterministicRandomUnstakesForAddress(address).map(
		(un): StakePosition => ({
			...un,
		}),
	)

export const deterministicRandomTxHistoryWithInput = (
	input: TransactionHistoryRequestInput,
): TransactionHistory => {
	const address = input.address
	const anInt: () => number = detPRNGWithPubKey(address.publicKey)
	const pubKeyBytes = address.publicKey
		.asData({ compressed: true })
		.slice(1, 33)
	const detRandomAddress = (): AccountAddressT =>
		castOfCharacters[anInt() % castOfCharacters.length]
	const detRandomValidatorAddress = detRandomValidatorAddressWithPRNG(anInt)
	const tokenAndAmounts = detRandBalanceOfTokenWithInfo(anInt)

	const deterministicRandomExecutedTransactions = (): ExecutedTransaction[] =>
		Array(input.size)
			.fill(undefined)
			.map(
				(_, index): ExecutedTransaction => {
					const bytesFromIndex = Buffer.allocUnsafe(2)
					bytesFromIndex.writeUInt16BE(index)
					const txIDBuffer = sha256(
						Buffer.concat([pubKeyBytes, bytesFromIndex]),
					)
					const date = new Date('2020-03-14T15:32:05')
					date.setMonth(index % 12)

					const txID = TransactionIdentifier.create(
						txIDBuffer,
					)._unsafeUnwrap()

					const detMakeActionForTx = (): ExecutedAction[] => {
						// mock max 5 actions per tx in history, min 1.
						const actionCount = Math.max(anInt() % 5, 1)
						return Array(actionCount)
							.fill(undefined)
							.map(
								(_, actionIndex): ExecutedAction => {
									const v: number = anInt() % 4 // Transfer, Stake, Unstake, Other
									const actionType: ActionType =
										v === 0
											? ActionType.TOKEN_TRANSFER
											: v === 1
											? ActionType.STAKE_TOKENS
											: v === 2
											? ActionType.UNSTAKE_TOKENS
											: ActionType.OTHER

									let executedAction: ExecutedAction

									const tokenAndAmount = tokenAndAmounts[
										actionIndex % tokenAndAmounts.length
									]!

									switch (actionType) {
										case ActionType.OTHER:
											executedAction = {
												type: ActionType.OTHER,
											}
											break
										case ActionType.STAKE_TOKENS:
											executedAction = {
												type: ActionType.STAKE_TOKENS,
												from: address,
												amount: Amount.fromUnsafe(
													anInt(),
												)._unsafeUnwrap(),
												validator: detRandomValidatorAddress(),
											} as any
											break
										case ActionType.UNSTAKE_TOKENS:
											executedAction = {
												type: ActionType.UNSTAKE_TOKENS,
												from: address,
												amount: Amount.fromUnsafe(
													anInt(),
												)._unsafeUnwrap(),
												validator: detRandomValidatorAddress(),
											} as any
											break
										case ActionType.TOKEN_TRANSFER:
											executedAction = {
												type: ActionType.TOKEN_TRANSFER,
												from_account: address.toString(),
												to_account: detRandomAddress().toString(),
												amount: tokenAndAmount.amount,
												rri: tokenAndAmount.token.rri,
											}
											break
									}

									return executedAction
								},
							)
					}

					const rndTxTypeInt = anInt() % 3
					const transactionType =
						rndTxTypeInt === 0
							? TransactionType.INCOMING
							: rndTxTypeInt === 1
							? TransactionType.FROM_ME_TO_ME
							: TransactionType.OUTGOING

					return {
						txID,
						sentAt: date,
						transactionType,
						fee: Amount.fromUnsafe(anInt())._unsafeUnwrap(),
						// message?: {
						// 	msg: string
						// 	encryptionScheme: string
						// }
						actions: detMakeActionForTx(),
					} as any
				},
			)

	const updatedCursor = sha256(
		input.cursor !== undefined ? Buffer.from(input.cursor) : pubKeyBytes,
	).toString('hex')

	return {
		cursor: updatedCursor,
		transactions: deterministicRandomExecutedTransactions(),
	}
}

const deterministicRandomLookupTXUsingHist = (
	txID: TransactionIdentifierT,
): SimpleExecutedTransaction => {
	const seed = sha256(Buffer.from(txID.__hex, 'hex'))
	const addressWithTXIdBytesAsSeed = AccountAddress.fromPublicKeyAndNetwork({
		publicKey: PrivateKey.fromBuffer(seed)._unsafeUnwrap().publicKey(),
		network: Network.MAINNET,
	})
	const txs = deterministicRandomTxHistoryWithInput({
		size: 1,
		address: addressWithTXIdBytesAsSeed,
	}).transactions
	if (txs.length === 0) {
		throw new Error('Expected at least one tx...')
	}
	return {
		...txs[0],
		txID,
	}
}

export const deterministicRandomBalances = (
	address: AccountAddressT,
): Observable<SimpleTokenBalances> =>
	of(deterministicRandomBalancesForAddress(address))

export const deterministicRandomTXHistory = (
	input: TransactionHistoryRequestInput,
): Observable<TransactionHistory> =>
	of(deterministicRandomTxHistoryWithInput(input))

export const deterministicRandomLookupTX = (
	txID: TransactionIdentifierT,
): Observable<SimpleExecutedTransaction> =>
	of(deterministicRandomLookupTXUsingHist(txID))

export const deterministicRandomUnstakesForAddr = (
	address: AccountAddressT,
): Observable<UnstakePositions> =>
	of(deterministicRandomUnstakesForAddress(address))

export const deterministicRandomStakesForAddr = (
	address: AccountAddressT,
): Observable<StakePositions> =>
	of(deterministicRandomStakesForAddress(address))

export const makeThrowingRadixCoreAPI = (nodeUrl?: string): any => ({
	node: { url: new URL(nodeUrl ?? 'https://www.radixdlt.com/') },

	networkId: (): Observable<Network> => {
		throw Error('Not implemented')
	},

	tokenBalancesForAddress: (_address: AccountAddressT): Observable<any> => {
		throw Error('Not implemented')
	},

	lookupTransaction: (_txID: any): Observable<any> => {
		throw Error('Not implemented')
	},

	validators: (_input: any): Observable<any> => {
		throw Error('Not implemented')
	},

	lookupValidator: (_input: ValidatorAddressT): Observable<any> => {
		throw Error('Not implemented')
	},

	transactionHistory: (
		_input: TransactionHistoryRequestInput,
	): Observable<any> => {
		throw Error('Not implemented')
	},

	nativeToken: (): Observable<any> => {
		throw Error('Not implemented')
	},

	tokenInfo: (_rri: ResourceIdentifierT): Observable<any> => {
		throw Error('Not implemented')
	},

	stakesForAddress: (_address: AccountAddressT): Observable<any> => {
		throw Error('Not implemented')
	},

	unstakesForAddress: (_address: AccountAddressT): Observable<any> => {
		throw Error('Not implemented')
	},

	transactionStatus: (
		_txID: TransactionIdentifierT,
	): Observable<StatusOfTransaction> => {
		throw Error('Not implemented')
	},

	NetworkTransactionThroughput: (): Observable<NetworkTransactionThroughput> => {
		throw Error('Not implemented')
	},

	NetworkTransactionDemand: (): Observable<NetworkTransactionDemand> => {
		throw Error('Not implemented')
	},

	buildTransaction: (
		_transactionIntent: TransactionIntent,
	): Observable<any> => {
		throw Error('Not implemented')
	},

	submitSignedTransaction: (_signedTransaction: any): Observable<any> => {
		throw Error('Not implemented')
	},

	finalizeTransaction: (
		_signedUnconfirmedTransaction: SignedTransaction,
	): Observable<FinalizedTransaction> => {
		throw Error('Not implemented')
	},
})

let txStatusMapCounter: Map<
	TransactionIdentifierT,
	number
> = (undefined as unknown) as Map<TransactionIdentifierT, number>

export const mockRadixCoreAPI = (
	input?: Readonly<{
		nodeUrl?: string
		network?: Network
	}>,
): any => {
	txStatusMapCounter = new Map<TransactionIdentifierT, number>()
	return {
		node: { url: new URL(input?.nodeUrl ?? 'https://www.radixdlt.com/') },

		networkId: (): Observable<Network> =>
			of(input?.network ?? Network.MAINNET).pipe(shareReplay(1)),
		nativeToken: (): Observable<Token> => of(xrd),
		tokenInfo: (rri: ResourceIdentifierT): Observable<Token> =>
			of(tokenByRRIMap.get(rri) ?? __fallBackAlexToken),
		tokenBalancesForAddress: deterministicRandomBalances,
		transactionStatus: (
			txID: TransactionIdentifierT,
		): Observable<StatusOfTransaction> => {
			const last = txStatusMapCounter.get(txID) ?? 0
			const incremented = last + 1
			txStatusMapCounter.set(txID, incremented)

			const status: TransactionStatus =
				last <= 1
					? TransactionStatus.PENDING
					: TransactionStatus.CONFIRMED

			return of({
				txID,
				status, // when TransactionStatus.FAIL ?
			})
		},
		validators: (input: ValidatorsRequestInput): Observable<Validators> =>
			of({
				cursor: 'cursor',
				validators: randomValidatorList(input.size),
			}),
		lookupValidator: (
			validatorAddress: ValidatorAddressT,
		): Observable<Validator> => {
			const validatorRnd = randomValidatorList(1, validatorAddress)[0]
			const validator: Validator = {
				...validatorRnd,
				address: validatorAddress,
			}
			return of(validator)
		},
		buildTransaction: (
			transactionIntent: TransactionIntent,
		): Observable<BuiltTransaction> =>
			of(randomUnsignedTransaction(transactionIntent)),
		finalizeTransaction: (
			signedTransaction: SignedTransaction,
		): Observable<FinalizedTransaction> =>
			of(detRandomSignedUnconfirmedTransaction(signedTransaction)),
		submitSignedTransaction: (signedUnconfirmedTX: any) =>
			of(signedUnconfirmedTX),
		NetworkTransactionDemand: (): Observable<NetworkTransactionDemand> =>
			of(randomDemand()),
		NetworkTransactionThroughput: (): Observable<NetworkTransactionThroughput> =>
			of(randomThroughput()),
		transactionHistory: deterministicRandomTXHistory,
		lookupTransaction: deterministicRandomLookupTX,
		unstakesForAddress: deterministicRandomUnstakesForAddr,
		stakesForAddress: deterministicRandomStakesForAddr,
	}
}

export const mockedAPI: Observable<any> = of(mockRadixCoreAPI())
