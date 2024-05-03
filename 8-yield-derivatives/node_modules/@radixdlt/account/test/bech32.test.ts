import { Bech32 } from '../src'

describe('bech32', () => {
	it('to from bech32 data', () => {
		const plaintext = 'Hello Radix!'
		const bech32DataHex = '09011216181b030f04010906021903090f001010'
		const bech32Data = Buffer.from(bech32DataHex, 'hex')
		const decodedBech32Data = Bech32.convertDataFromBech32(
			bech32Data,
		)._unsafeUnwrap()
		expect(decodedBech32Data.toString('utf8')).toBe(plaintext)

		const convertedToBech32Data = Bech32.convertDataToBech32(
			Buffer.from(plaintext, 'utf8'),
		)._unsafeUnwrap()

		expect(convertedToBech32Data.toString('hex')).toBe(bech32DataHex)
	})
})
