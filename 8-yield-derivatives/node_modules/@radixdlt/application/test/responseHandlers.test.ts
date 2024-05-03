import { handleAccountBalancesResponse } from '../src/api/open-api/responseHandlers'

const fixtures = [
	{
		data: {
			ledger_state: {
				version: 42905709,
				timestamp: '2022-01-06T07:36:32.638Z',
				epoch: 4405,
				round: 9787,
			},
			account_balances: {
				staked_and_unstaking_balance: {
					value: '593224304698885819581',
					token_identifier: {
						rri: 'xrd_tr1qyf0x76s',
					},
				},
				liquid_balances: [
					{
						value: '100000000000000000000000',
						token_identifier: {
							rri:
								'fire_tr1qvs4gje6qfxmu5wfn9jd5x9ku20ds7fcucn6tzcnyxwq7n02zx',
						},
					},
					{
						value: '99999894000000100000000',
						token_identifier: {
							rri:
								'buzzsaw_tr1q0aymplntjgcdsc5fuxcgq9me47yu26qf929cqexduxs7c299n',
						},
					},
					{
						value: '99971993105302200000000',
						token_identifier: {
							rri:
								'captainfr33domst0000000000000ken_tr1q09jf8c05v3lfj3tqc04x7nlp0sag8yq5k6qpaexxnrs004s7q',
						},
					},
					{
						value: '909699413989940263553',
						token_identifier: {
							rri: 'xrd_tr1qyf0x76s',
						},
					},
					{
						value: '5999999999999999999',
						token_identifier: {
							rri:
								'sptve_tr1qvddj7vg004cstsqvu6nar0ssr4x8v2r7rnry2e66n7q0nmaus',
						},
					},
				],
			},
		},
	} as any,
]

describe('handleAccountBalancesResponse', () => {
	it('should correctly transform balances', () => {
		const actual = handleAccountBalancesResponse(
			fixtures[0],
		)._unsafeUnwrap()

		const expectedLiquidBalances =
			fixtures[0].data.account_balances.liquid_balances

		const actualLiquidBalances = actual.account_balances.liquid_balances.map(
			item => ({
				value: item.value.toString(),
				token_identifier: { rri: item.token_identifier.rri.toString() },
			}),
		)

		const expectedStakedBalance =
			fixtures[0].data.account_balances.staked_and_unstaking_balance

		const actualStakedBalance = {
			value: actual.account_balances.staked_and_unstaking_balance.value.toString(),
			token_identifier: {
				rri: actual.account_balances.staked_and_unstaking_balance.token_identifier.rri.toString(),
			},
		}

		const liquidXrdValue = actual.account_balances.liquid_balances.find(
			item => item.token_identifier.rri.toString() === 'xrd_tr1qyf0x76s',
		)?.value
		const stakedXrdValue =
			actual.account_balances.staked_and_unstaking_balance.value

		expect(actualLiquidBalances).toEqual(expectedLiquidBalances)
		expect(actualStakedBalance).toEqual(expectedStakedBalance)
		expect('1502923718688826083134').toEqual(
			liquidXrdValue?.add(stakedXrdValue).toString(),
		)
	})
})
