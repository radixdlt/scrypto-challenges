import {
	SigningKeychainT,
	SigningKeyT,
	SigningKeysT,
	AccountAddressT,
	DeriveNextInput,
	AccountAddress,
	DeriveHWSigningKeyInput,
} from '@radixdlt/account'
import {
	WalletT,
	AccountT,
	AccountsT,
	SwitchAccountInput,
	SwitchToAccount,
	AddAccountByPrivateKeyInput,
} from './_types'
import { Observable, of, throwError } from 'rxjs'
import { Account, isAccount } from './account'
import { map, mergeMap } from 'rxjs/operators'
import { Option } from 'prelude-ts'
import { PublicKeyT, HDPathRadixT } from '@radixdlt/crypto'
import { Network } from '@radixdlt/primitives'
import { log } from '@radixdlt/util/dist/logging'

const create = (
	input: Readonly<{
		signingKeychain: SigningKeychainT
		network: Network
	}>,
): WalletT => {
	const { network, signingKeychain } = input
	const skToAccountAddress = (signingKey: SigningKeyT): AccountAddressT =>
		AccountAddress.fromPublicKeyAndNetwork({
			network,
			publicKey: signingKey.publicKey,
		})

	const skToAccount = (signingKey: SigningKeyT): AccountT =>
		Account.create({ signingKey, address: skToAccountAddress(signingKey) })

	const sksToAccounts = (signingKeys: SigningKeysT): AccountsT => {
		const getAccountWithHDSigningKeyByHDPath = (
			hdPath: HDPathRadixT,
		): Option<AccountT> =>
			signingKeys.getHDSigningKeyByHDPath(hdPath).map(skToAccount)

		const getAnyAccountByPublicKey = (
			publicKey: PublicKeyT,
		): Option<AccountT> =>
			signingKeys.getAnySigningKeyByPublicKey(publicKey).map(skToAccount)

		const all = signingKeys.all.map(skToAccount)

		return {
			all,
			getAccountWithHDSigningKeyByHDPath,
			getAnyAccountByPublicKey,
			accountsWithHDSigningKeys: () =>
				signingKeys.hdSigningKeys().map(skToAccount),
			accountsWithHardwareHDSigningKeys: () =>
				signingKeys.hardwareHDSigningKeys().map(skToAccount),
			accountsWithLocalHDSigningKeys: () =>
				signingKeys.localHDSigningKeys().map(skToAccount),
			accountsWithNonHDSigningKeys: () =>
				signingKeys.nonHDSigningKeys().map(skToAccount),
			size: () => all.length,
		}
	}

	const observeActiveAccount = (): Observable<AccountT> =>
		signingKeychain.observeActiveSigningKey().pipe(map(skToAccount))

	return {
		__unsafeGetAccount: (): AccountT =>
			skToAccount(signingKeychain.__unsafeGetSigningKey()),

		revealMnemonic: signingKeychain.revealMnemonic,

		deriveNextLocalHDAccount: (
			input?: DeriveNextInput,
		): Observable<AccountT> =>
			signingKeychain
				.deriveNextLocalHDSigningKey(input)
				.pipe(map(skToAccount)),

		deriveHWAccount: (
			input: DeriveHWSigningKeyInput,
		): Observable<AccountT> =>
			signingKeychain.deriveHWSigningKey(input).pipe(map(skToAccount)),

		displayAddressForActiveHWAccountOnHWDeviceForVerification: (): Observable<void> =>
			signingKeychain
				.__unsafeGetSigningKey()
				.getPublicKeyDisplayOnlyAddress()
				.pipe(
					mergeMap(
						(): Observable<void> => {
							return of(undefined)
						},
					),
				),

		observeActiveAccount,
		observeAccounts: (): Observable<AccountsT> =>
			signingKeychain.observeSigningKeys().pipe(map(sksToAccounts)),

		addAccountFromPrivateKey: (
			input: AddAccountByPrivateKeyInput,
		): Observable<AccountT> =>
			of(skToAccount(signingKeychain.addSigningKeyFromPrivateKey(input))),

		restoreLocalHDAccountsToIndex: (index: number): Observable<AccountsT> =>
			signingKeychain
				.restoreLocalHDSigningKeysUpToIndex(index)
				.pipe(map(sksToAccounts)),

		switchAccount: (input: SwitchAccountInput): AccountT => {
			const isSwitchToAccount = (
				something: unknown,
			): something is SwitchToAccount => {
				const inspection = input as SwitchToAccount
				return (
					inspection.toAccount !== undefined &&
					isAccount(inspection.toAccount)
				)
			}

			if (isSwitchToAccount(input)) {
				return skToAccount(
					signingKeychain.switchSigningKey({
						toSigningKey: input.toAccount.signingKey,
					}),
				)
			} else {
				return skToAccount(signingKeychain.switchSigningKey(input))
			}
		},
	}
}

export const Wallet = {
	create,
}
