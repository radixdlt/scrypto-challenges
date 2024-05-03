/*import { ResourceIdentifier } from '@radixdlt/account'
import { Amount, Network } from '@radixdlt/primitives'
import { IntendedTransferTokens, TransferTokensInput, alice, bob } from '../src'

describe('TransferTokensActions', () => {
	const resourceIdentifier = ResourceIdentifier.fromPublicKeyAndNameAndNetwork(
		{
			publicKey: alice.publicKey,
			name: 'foobar',
			network: Network.MAINNET,
		},
	)._unsafeUnwrap()
	const amount = Amount.fromUnsafe(6)._unsafeUnwrap()

	const input = <TransferTokensInput>{
		to: bob,
		amount: amount,
		tokenIdentifier: resourceIdentifier,
	}

	it(`should have a 'recipient' equal to 'input.to'.`, () => {
		const tokenTransfer = IntendedTransferTokens.create(
			input,
			alice,
		)._unsafeUnwrap()
		expect(tokenTransfer.to.equals(bob)).toBe(true)
	})

	it(`should have an 'amount' equal to 'input.amount'.`, () => {
		const tokenTransfer = IntendedTransferTokens.create(
			input,
			alice,
		)._unsafeUnwrap()
		expect(tokenTransfer.amount.eq(amount)).toBe(true)
	})

	it(`should have a 'sender' equal to 'input.from'.`, () => {
		const tokenTransfer = IntendedTransferTokens.create(
			input,
			alice,
		)._unsafeUnwrap()
		expect(tokenTransfer.from.equals(alice)).toBe(true)
	})

	it('should be possible to transfer 0 tokens', () => {
		const zero = Amount.fromUnsafe(0)._unsafeUnwrap()
		const tokenTransfer = IntendedTransferTokens.create(
			{ ...input, amount: zero },
			alice,
		)._unsafeUnwrap()

		expect(tokenTransfer.amount.eq(zero)).toBe(true)
	})
})

*/
