import {
	BehaviorSubject,
	combineLatest,
	Observable,
	of,
	ReplaySubject,
	Subscription,
	throwError,
} from 'rxjs'
import { SigningKey, isSigningKey } from './signingKey'
import {
	SigningKeysT,
	SigningKeyT,
	DeriveNextInput,
	SwitchSigningKeyInput,
	SwitchToSigningKey,
	SwitchToIndex,
	AddSigningKeyByPrivateKeyInput,
	SigningKeychainT,
	DeriveHWSigningKeyInput,
	Signing,
} from './_types'
import { map, mergeMap, shareReplay, take, tap } from 'rxjs/operators'
import {
	Keystore,
	KeystoreT,
	PublicKeyT,
	SignatureT,
	HDPathRadix,
	HDPathRadixT,
	Int32,
	HDMasterSeed,
	MnemomicT,
	Mnemonic,
} from '@radixdlt/crypto'
import { Option } from 'prelude-ts'
import { arraysEqual, log, msgFromError } from '@radixdlt/util'
import { ResultAsync } from 'neverthrow'
import { HardwareSigningKeyT, HardwareWalletT } from '@radixdlt/hardware-wallet'
import { BuiltTransactionReadyToSign } from '@radixdlt/primitives'

const stringifySigningKeysArray = (signingKeys: SigningKeyT[]): string =>
	signingKeys.map(a => a.toString()).join(',\n')

const stringifySigningKeys = (signingKeys: SigningKeysT): string => {
	const allSigningKeysString = stringifySigningKeysArray(signingKeys.all)

	return `
		size: ${signingKeys.size()},
		#hdSigningKeys: ${signingKeys.hdSigningKeys().length},
		#nonHDSigningKeys: ${signingKeys.nonHDSigningKeys().length},
		#localHDSigningKeys: ${signingKeys.localHDSigningKeys().length},
		#hardwareHDSigningKeys: ${signingKeys.hardwareHDSigningKeys().length},
		
		all: ${allSigningKeysString}
	`
}

type MutableSigningKeysT = SigningKeysT &
	Readonly<{
		add: (signingKey: SigningKeyT) => void
	}>

const createSigningKeys = (_all: SigningKeyT[]): MutableSigningKeysT => {
	const all: SigningKeyT[] = []

	const getHDSigningKeyByHDPath = (
		hdPath: HDPathRadixT,
	): Option<SigningKeyT> => {
		const signingKey = all
			.filter(a => a.isHDSigningKey)
			.find(a => a.hdPath!.equals(hdPath))
		return Option.of(signingKey)
	}

	const getAnySigningKeyByPublicKey = (
		publicKey: PublicKeyT,
	): Option<SigningKeyT> => {
		const signingKey = all.find(a => a.publicKey.equals(publicKey))
		return Option.of(signingKey)
	}

	const localHDSigningKeys = () => all.filter(a => a.isLocalHDSigningKey)
	const hardwareHDSigningKeys = () => all.filter(a => a.isHardwareSigningKey)
	const nonHDSigningKeys = () => all.filter(a => !a.isHDSigningKey)
	const hdSigningKeys = () => all.filter(a => a.isHDSigningKey)

	const add = (signingKey: SigningKeyT): void => {
		if (
			all.find(a => a.type.uniqueKey === signingKey.type.uniqueKey) !==
			undefined
		) {
			// already there
			return
		}
		// new
		all.push(signingKey)
	}

	const signingKeys: MutableSigningKeysT = {
		toString: (): string => {
			throw new Error('Overriden below')
		},
		equals: (other: SigningKeysT): boolean => arraysEqual(other.all, all),
		add,
		localHDSigningKeys,
		hardwareHDSigningKeys,
		nonHDSigningKeys,
		hdSigningKeys,
		all,
		size: () => all.length,
		getHDSigningKeyByHDPath,
		getAnySigningKeyByPublicKey,
	}

	return {
		...signingKeys,
		toString: (): string => stringifySigningKeys(signingKeys),
	}
}

export const isSwitchToIndex = (
	something: unknown,
): something is SwitchToIndex => {
	const inspection = something as SwitchToIndex
	return inspection.toIndex !== undefined
}

const MutableSigningKeys = {
	create: createSigningKeys,
}

const create = (
	input: Readonly<{
		mnemonic: MnemomicT
		startWithInitialSigningKey?: boolean
	}>,
): SigningKeychainT => {
	const subs = new Subscription()
	const { mnemonic } = input
	const startWithInitialSigningKey = input.startWithInitialSigningKey ?? true
	const masterSeed = HDMasterSeed.fromMnemonic({ mnemonic })
	const hdNodeDeriverWithBip32Path = masterSeed.masterNode().derive

	let unsafeActiveSigningKey: SigningKeyT = (undefined as unknown) as SigningKeyT
	const activeSigningKeySubject = new ReplaySubject<SigningKeyT>()
	const setActiveSigningKey = (newSigningKey: SigningKeyT): void => {
		activeSigningKeySubject.next(newSigningKey)
		unsafeActiveSigningKey = newSigningKey
	}

	const signingKeysSubject = new BehaviorSubject<MutableSigningKeysT>(
		MutableSigningKeys.create([]),
	)

	const revealMnemonic = (): MnemomicT => mnemonic

	const numberOfAllSigningKeys = (): number =>
		signingKeysSubject.getValue().size()
	const numberOfLocalHDSigningKeys = (): number =>
		signingKeysSubject.getValue().localHDSigningKeys().length

	const numberOfHWSigningKeys = (): number =>
		signingKeysSubject.getValue().hardwareHDSigningKeys().length

	const _addAndMaybeSwitchToNewSigningKey = (
		newSigningKey: SigningKeyT,
		alsoSwitchTo?: boolean,
	): SigningKeyT => {
		const alsoSwitchTo_ = alsoSwitchTo ?? false
		const signingKeys = signingKeysSubject.getValue()
		signingKeys.add(newSigningKey)
		signingKeysSubject.next(signingKeys)
		if (alsoSwitchTo_) {
			setActiveSigningKey(newSigningKey)
		}
		return newSigningKey
	}

	const deriveHWSigningKey = (
		input: DeriveHWSigningKeyInput,
	): Observable<SigningKeyT> => {
		const nextPath = (): HDPathRadixT => {
			const index = numberOfHWSigningKeys()
			return HDPathRadix.create({
				address: { index, isHardened: true },
			})
		}
		const hdPath: HDPathRadixT =
			input.keyDerivation === 'next' ? nextPath() : input.keyDerivation

		return input.hardwareWalletConnection.pipe(
			take(1),
			mergeMap(
				(
					hardwareWallet: HardwareWalletT,
				): Observable<HardwareSigningKeyT> =>
					hardwareWallet.makeSigningKey(
						hdPath,
						input.verificationPrompt,
					),
			),
			map((hardwareSigningKey: HardwareSigningKeyT) => {
				const signingKey = SigningKey.fromHDPathWithHWSigningKey({
					hdPath,
					hardwareSigningKey,
				})
				_addAndMaybeSwitchToNewSigningKey(
					signingKey,
					input.alsoSwitchTo,
				)
				return signingKey
			}),
		)
	}

	const _deriveLocalHDSigningKeyWithPath = (
		input: Readonly<{
			hdPath: HDPathRadixT
			alsoSwitchTo?: boolean // defaults to false
		}>,
	): Observable<SigningKeyT> => {
		const { hdPath } = input

		const newSigningKey = _addAndMaybeSwitchToNewSigningKey(
			SigningKey.byDerivingNodeAtPath({
				hdPath,
				deriveNodeAtPath: () => hdNodeDeriverWithBip32Path(hdPath),
			}),
			input.alsoSwitchTo,
		)

		return of(newSigningKey)
	}

	const _deriveNextLocalHDSigningKeyAtIndex = (
		input: Readonly<{
			addressIndex: Readonly<{
				index: Int32
				isHardened?: boolean // defaults to true
			}>
			alsoSwitchTo?: boolean // defaults to false
		}>,
	): Observable<SigningKeyT> =>
		_deriveLocalHDSigningKeyWithPath({
			hdPath: HDPathRadix.create({
				address: input.addressIndex,
			}),
			alsoSwitchTo: input.alsoSwitchTo,
		})

	const deriveNextLocalHDSigningKey = (
		input?: DeriveNextInput,
	): Observable<SigningKeyT> => {
		const index = numberOfLocalHDSigningKeys()
		return _deriveNextLocalHDSigningKeyAtIndex({
			addressIndex: {
				index,
				isHardened: input?.isHardened ?? true,
			},
			alsoSwitchTo: input?.alsoSwitchTo,
		})
	}

	const switchSigningKey = (input: SwitchSigningKeyInput): SigningKeyT => {
		const isSwitchToSigningKey = (
			something: unknown,
		): something is SwitchToSigningKey => {
			const inspection = input as SwitchToSigningKey
			return (
				inspection.toSigningKey !== undefined &&
				isSigningKey(inspection.toSigningKey)
			)
		}

		if (input === 'last') {
			const lastIndex = numberOfAllSigningKeys() - 1
			return switchSigningKey({ toIndex: lastIndex })
		} else if (input === 'first') {
			return switchSigningKey({ toIndex: 0 })
		} else if (isSwitchToSigningKey(input)) {
			const toSigningKey = input.toSigningKey
			setActiveSigningKey(toSigningKey)
			log.info(
				`Active signingKey switched to: ${toSigningKey.toString()}`,
			)
			return toSigningKey
		} else if (isSwitchToIndex(input)) {
			const unsafeTargetIndex = input.toIndex
			const signingKeys = signingKeysSubject.getValue()

			const safeTargetIndex = Math.min(
				unsafeTargetIndex,
				signingKeys.size(),
			)

			const firstSigningKey = Array.from(signingKeys.all)[safeTargetIndex]
			if (!firstSigningKey) {
				const err = `No signingKeys.`
				log.error(err)
				throw new Error(err)
			}
			return switchSigningKey({ toSigningKey: firstSigningKey })
		} else {
			const err = `Incorrect implementation, failed to type check 'input' of switchSigningKey. Probably is 'isSigningKey' typeguard wrong.`
			log.error(err)
			throw new Error(err)
		}
	}

	if (startWithInitialSigningKey) {
		subs.add(
			deriveNextLocalHDSigningKey({
				alsoSwitchTo: true,
			}).subscribe(),
		)
	}

	const activeSigningKey$ = activeSigningKeySubject.asObservable()

	const signingKeys$ = signingKeysSubject.asObservable().pipe(shareReplay())

	const restoreLocalHDSigningKeysUpToIndex = (
		index: number,
	): Observable<SigningKeysT> => {
		if (index < 0) {
			const errMsg = `targetIndex must not be negative`
			console.error(errMsg)
			return throwError(new Error(errMsg))
		}

		const localHDSigningKeysSize = numberOfLocalHDSigningKeys()
		const numberOfSigningKeysToCreate = index - localHDSigningKeysSize
		if (numberOfSigningKeysToCreate < 0) {
			return signingKeys$
		}

		const signingKeysObservableList: Observable<SigningKeyT>[] = Array(
			numberOfSigningKeysToCreate,
		)
			.fill(undefined)
			.map((_, index) =>
				_deriveNextLocalHDSigningKeyAtIndex({
					addressIndex: { index: localHDSigningKeysSize + index },
				}),
			)

		return combineLatest(signingKeysObservableList).pipe(
			mergeMap(_ => signingKeys$),
			take(1),
		)
	}

	const addSigningKeyFromPrivateKey = (
		input: AddSigningKeyByPrivateKeyInput,
	): SigningKeyT => {
		const signingKey = SigningKey.fromPrivateKey(input)
		_addAndMaybeSwitchToNewSigningKey(signingKey, input.alsoSwitchTo)
		return signingKey
	}

	return {
		revealMnemonic,
		// should only be used for testing
		__unsafeGetSigningKey: (): SigningKeyT => unsafeActiveSigningKey,
		deriveNextLocalHDSigningKey,
		deriveHWSigningKey,
		switchSigningKey,
		restoreLocalHDSigningKeysUpToIndex,
		addSigningKeyFromPrivateKey,
		observeSigningKeys: (): Observable<SigningKeysT> => signingKeys$,
		observeActiveSigningKey: (): Observable<SigningKeyT> =>
			activeSigningKey$,
		sign: (
			tx: BuiltTransactionReadyToSign,
			nonXrdHRP?: string,
		): Observable<SignatureT> =>
			activeSigningKey$.pipe(mergeMap(a => a.sign(tx, nonXrdHRP))),
		signHash: (hashedMessage: Buffer): Observable<SignatureT> =>
			activeSigningKey$.pipe(mergeMap(a => a.signHash(hashedMessage))),
	}
}

const byLoadingAndDecryptingKeystore = (
	input: Readonly<{
		password: string
		load: () => Promise<KeystoreT>
		startWithInitialSigningKey?: boolean
	}>,
): ResultAsync<SigningKeychainT, Error> => {
	const loadKeystore = (): ResultAsync<KeystoreT, Error> =>
		ResultAsync.fromPromise(input.load(), (e: unknown) => {
			const underlyingError = msgFromError(e)
			const errMsg = `Failed to load keystore, underlying error: '${underlyingError}'`
			log.error(errMsg)
			return new Error(errMsg)
		})
	return loadKeystore()
		.map((keystore: KeystoreT) => {
			log.info('Keystore successfully loaded.')
			return { ...input, keystore }
		})
		.andThen(SigningKeychain.fromKeystore)
}

const fromKeystore = (
	input: Readonly<{
		keystore: KeystoreT
		password: string
		startWithInitialSigningKey?: boolean
	}>,
): ResultAsync<SigningKeychainT, Error> =>
	Keystore.decrypt(input)
		.map(entropy => ({ entropy }))
		.andThen(Mnemonic.fromEntropy)
		.map(mnemonic => ({
			mnemonic,
			startWithInitialSigningKey: input.startWithInitialSigningKey,
		}))
		.map(create)

const byEncryptingMnemonicAndSavingKeystore = (
	input: Readonly<{
		mnemonic: MnemomicT
		password: string
		save: (keystoreToSave: KeystoreT) => Promise<void>
		startWithInitialSigningKey?: boolean
	}>,
): ResultAsync<SigningKeychainT, Error> => {
	const { mnemonic, password, startWithInitialSigningKey } = input

	const save = (keystoreToSave: KeystoreT): ResultAsync<KeystoreT, Error> =>
		ResultAsync.fromPromise(input.save(keystoreToSave), (e: unknown) => {
			const underlyingError = msgFromError(e)
			const errMsg = `Failed to save keystore, underlying error: '${underlyingError}'`
			log.error(errMsg)
			return new Error(errMsg)
		}).map(() => {
			log.info('Keystore successfully saved.')
			return keystoreToSave
		})

	return Keystore.encryptSecret({
		secret: mnemonic.entropy,
		password,
	})
		.andThen(save)
		.map((keystore: KeystoreT) => ({
			keystore,
			password,
			startWithInitialSigningKey,
		}))
		.andThen(SigningKeychain.fromKeystore)
}

export const SigningKeychain = {
	create,
	fromKeystore,
	byLoadingAndDecryptingKeystore,
	byEncryptingMnemonicAndSavingKeystore,
}
