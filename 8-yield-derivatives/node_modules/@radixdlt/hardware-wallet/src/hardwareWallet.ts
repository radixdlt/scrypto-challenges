import { HardwareSigningKeyT, HardwareWalletT, SignHashInput } from './_types'
import { Observable } from 'rxjs'
import {
	ECPointOnCurveT,
	HDPathRadix,
	HDPathRadixT,
	PublicKeyT,
	SignatureT,
} from '@radixdlt/crypto'
import { map } from 'rxjs/operators'
import { BuiltTransactionReadyToSign } from '@radixdlt/primitives'

export const path000H = HDPathRadix.create({
	address: { index: 0, isHardened: true },
})

export type HardwareWalletWithoutSK = Omit<HardwareWalletT, 'makeSigningKey'>

export const signingKeyWithHardWareWallet = (
	hardwareWallet: HardwareWalletWithoutSK,
	path: HDPathRadixT,
	verificationPrompt: boolean = true,
): Observable<HardwareSigningKeyT> =>
	hardwareWallet
		.getPublicKey({
			path,
			display: verificationPrompt,
			// display BIP32 path as part of derivation call.
			verifyAddressOnly: false,
		})
		.pipe(
			map((publicKey: PublicKeyT) => ({
				publicKey,
				getPublicKeyDisplayOnlyAddress: (): Observable<PublicKeyT> => {
					return hardwareWallet.getPublicKey({
						path,
						display: true,
						// Omits BIP32 path screen on Ledger.
						verifyAddressOnly: true,
					})
				},
				signHash: (hashedMessage: Buffer): Observable<SignatureT> =>
					hardwareWallet.doSignHash({
						hashToSign: hashedMessage,
						path,
					}),
				sign: (
					tx: BuiltTransactionReadyToSign,
					nonXrdHRP?: string,
				): Observable<SignatureT> =>
					hardwareWallet
						.doSignTransaction({ tx, path, nonXrdHRP })
						.pipe(map(o => o.signature)),
				keyExchange: (
					publicKeyOfOtherParty: PublicKeyT,
					display?: 'encrypt' | 'decrypt',
				): Observable<ECPointOnCurveT> =>
					hardwareWallet.doKeyExchange({
						display,
						path,
						publicKeyOfOtherParty,
					}),
			})),
		)
