import { createWallet } from './util'
import { map, take, toArray } from 'rxjs/operators'
import { Subscription } from 'rxjs'

describe('wallet', () => {
	it('can observeActiveAccount', done => {
		const subs = new Subscription()
		const wallet = createWallet()

		const expectedValues = [0, 1, 2]

		subs.add(
			wallet
				.observeActiveAccount()
				.pipe(
					map(account => account.hdPath!.addressIndex.value()),
					take(expectedValues.length),
					toArray(),
				)
				.subscribe(
					values => {
						expect(values).toStrictEqual(expectedValues)
						done()
					},
					error => done(error),
				),
		)

		subs.add(
			wallet
				.deriveNextLocalHDAccount({ alsoSwitchTo: true })
				.subscribe(_ => {
					subs.add(
						wallet
							.deriveNextLocalHDAccount({ alsoSwitchTo: true })
							.subscribe(),
					)
				}),
		)
	})
})
