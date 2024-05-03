import {
	ActionInput,
	ActionType,
	ExecutedAction,
	IntendedAction,
	StakeTokensAction,
	StakeTokensInput,
	TransferTokensAction,
	TransferTokensInput,
	UnstakeTokensAction,
	UnstakeTokensInput,
} from '../actions'
import {
	TransactionIntent,
	TransactionIntentBuilderDoNotEncryptInput,
	TransactionIntentBuilderDoNotEncryptOption,
	TransactionIntentBuilderEncryptOption,
	TransactionIntentBuilderOptions,
	TransactionIntentBuilderState,
	TransactionIntentBuilderT,
} from './_types'
import {
	AccountAddress,
	AccountAddressT,
	isResourceIdentifier,
} from '../../../account'
import { isObservable, Observable, of, throwError } from 'rxjs'
import { map, mergeMap } from 'rxjs/operators'
import {
	IntendedTransferTokens,
	isTransferTokensInput,
	IntendedStakeTokens,
	isStakeTokensInput,
	IntendedUnstakeTokens,
} from '../actions'
import { combine, err, ok, Result } from 'neverthrow'
import {
	EncryptedMessageT,
	Message,
	MessageEncryption,
	PublicKeyT,
} from '../../../crypto'
import { Option } from 'prelude-ts'
import { isAmount } from '../../../primitives'
import { log, toObservableFromResult } from '../../../util'
import { AccountT, MessageInTransaction } from '../_types'

type IntendedActionsFrom = Readonly<{
	intendedActions: IntendedAction[]
	from: AccountAddressT
}>

export const singleRecipientFromActions = (
	mine: PublicKeyT,
	actions: UserAction[],
): Result<PublicKeyT, Error> => {
	const others = flatMapAddressesOf({ actions })
		.map(a => a.publicKey)
		.filter(a => !a.equals(mine))

	if (others.length > 1) {
		const errMsg = `Cannot encrypt/decrypt message for a transaction containing more than one recipient addresses.`
		log.error(errMsg)
		throw new Error(errMsg)
	}

	const toSelf = others.length === 0
	if (toSelf) {
		log.debug(`Encrypted message is to oneself.`)
	}

	return ok(toSelf ? mine : others[0])
}

type ActorsInEncryption = {
	encryptingAccount: AccountT
	singleRecipientPublicKey: PublicKeyT
}

const ensureSingleRecipient = (
	input: Readonly<{
		intendedActionsFrom: IntendedActionsFrom
		encryptingAccount: AccountT
	}>,
): Observable<ActorsInEncryption> =>
	toObservableFromResult(
		singleRecipientFromActions(
			input.encryptingAccount.publicKey,
			input.intendedActionsFrom.intendedActions,
		),
	).pipe(
		map(singleRecipientPublicKey => ({
			encryptingAccount: input.encryptingAccount,
			singleRecipientPublicKey: singleRecipientPublicKey,
		})),
	)

type IntermediateAction = ActionInput & {
	type: 'transfer' | 'stake' | 'unstake'
}

const mustHaveAtLeastOneAction = new Error(
	'A transaction intent must contain at least one of the following actions: TransferToken, StakeTokens or UnstakeTokens',
)

export const isTransferTokensAction = (
	something: unknown,
): something is TransferTokensAction => {
	const inspection = something as TransferTokensAction
	return (
		inspection.type === ActionType.TOKEN_TRANSFER &&
		!!inspection.to_account &&
		!!inspection.from_account &&
		isAmount(inspection.amount) &&
		isResourceIdentifier(inspection.rri)
	)
}

export const isStakeTokensAction = (
	something: unknown,
): something is StakeTokensAction => {
	const inspection = something as StakeTokensAction
	return (
		inspection.type === ActionType.STAKE_TOKENS &&
		!!inspection.from_account &&
		!!inspection.to_validator &&
		isAmount(inspection.amount)
	)
}

export const isUnstakeTokensAction = (
	something: unknown,
): something is UnstakeTokensAction => {
	const inspection = something as UnstakeTokensAction
	return (
		inspection.type === ActionType.UNSTAKE_TOKENS &&
		!!inspection.from_validator &&
		!!inspection.to_account &&
		isAmount(inspection.amount)
	)
}

const decodeApiAddress = (address: string): AccountAddressT => {
	const result = AccountAddress.fromUnsafe(address)
	return result._unsafeUnwrap()
}

type UserAction = IntendedAction | ExecutedAction
export const getUniqueAddresses = (
	input: Readonly<{
		action: UserAction
		includeFrom?: boolean
		includeTo?: boolean
	}>,
): AccountAddressT[] => {
	const action = input.action
	const includeFrom = input.includeFrom ?? true
	const includeTo = input.includeTo ?? true
	if (isTransferTokensAction(action)) {
		const addresses: AccountAddressT[] = []
		if (includeTo) {
			addresses.push(decodeApiAddress(action.to_account))
		}
		if (includeFrom) {
			addresses.push(decodeApiAddress(action.from_account))
		}
		return addresses
	} else if (isStakeTokensAction(action)) {
		const addresses: AccountAddressT[] = []
		if (includeFrom) {
			addresses.push(decodeApiAddress(action.from_account))
		}
		return addresses
	} else if (isUnstakeTokensAction(action)) {
		const addresses: AccountAddressT[] = []
		if (includeFrom) {
			addresses.push(decodeApiAddress(action.to_account))
		}
		return addresses
	} else {
		return []
	}
}

export const flatMapAddressesOf = (
	input: Readonly<{
		actions: UserAction[]
		includeFrom?: boolean
		includeTo?: boolean
	}>,
): AccountAddressT[] => {
	const { actions, includeFrom, includeTo } = input
	const flatMapped = actions.reduce(
		(acc: AccountAddressT[], action: UserAction) => {
			const uniqueAddressOfAction = getUniqueAddresses({
				action,
				includeFrom,
				includeTo,
			})
			return acc.concat(...uniqueAddressOfAction)
		},
		[] as AccountAddressT[],
	)

	const set = new Set<string>()
	return flatMapped.filter(a => {
		const str = a.toString()
		const hasNt = !set.has(str)
		set.add(str)
		return hasNt
	})
}

const isTransactionIntentBuilderEncryptInput = (
	something: unknown,
): something is TransactionIntentBuilderEncryptOption => {
	const inspection = something as TransactionIntentBuilderEncryptOption
	return (
		inspection.encryptMessageIfAnyWithAccount !== undefined &&
		isObservable(inspection.encryptMessageIfAnyWithAccount) &&
		(inspection.spendingSender !== undefined
			? isObservable(inspection.spendingSender)
			: true)
	)
}

const isTransactionIntentBuilderDoNotEncryptInput = (
	something: unknown,
): something is TransactionIntentBuilderDoNotEncryptInput => {
	if (isTransactionIntentBuilderEncryptInput(something)) {
		return false
	}
	const inspection = something as TransactionIntentBuilderDoNotEncryptInput
	return (
		inspection.spendingSender !== undefined &&
		isObservable(inspection.spendingSender)
	)
}

const isTransactionIntentBuilderDoNotEncryptOption = (
	something: unknown,
): something is TransactionIntentBuilderDoNotEncryptOption => {
	const inspection = something as TransactionIntentBuilderDoNotEncryptOption
	return (
		inspection.skipEncryptionOfMessageIfAny !== undefined &&
		isTransactionIntentBuilderDoNotEncryptInput(
			inspection.skipEncryptionOfMessageIfAny,
		)
	)
}

const create = (): TransactionIntentBuilderT => {
	const intermediateActions: IntermediateAction[] = []
	let maybePlaintextMsgToEncrypt: Option<MessageInTransaction> = Option.none()
	const snapshotState = (): TransactionIntentBuilderState => ({
		actionInputs: intermediateActions,
		message: maybePlaintextMsgToEncrypt.getOrUndefined(),
	})

	const snapshotBuilderState = (): {
		__state: TransactionIntentBuilderState
	} => ({
		__state: snapshotState(),
	})

	const addAction = (
		input: ActionInput,
		type: 'transfer' | 'stake' | 'unstake',
	): TransactionIntentBuilderT => {
		intermediateActions.push({
			type,
			...input,
		})
		return {
			...methods,
			...snapshotBuilderState(),
		}
	}

	const transferTokens = (
		input: TransferTokensInput,
	): TransactionIntentBuilderT => addAction(input, 'transfer')

	const stakeTokens = (input: StakeTokensInput): TransactionIntentBuilderT =>
		addAction(input, 'stake')

	const unstakeTokens = (
		input: UnstakeTokensInput,
	): TransactionIntentBuilderT => addAction(input, 'unstake')

	const replaceAnyPreviousMessageWithNew = (
		newMessage: MessageInTransaction,
	): TransactionIntentBuilderT => {
		maybePlaintextMsgToEncrypt = Option.some(newMessage)
		return {
			...methods,
			...snapshotBuilderState(),
		}
	}

	const intendedActionsFromIntermediateActions = (
		from: AccountAddressT,
	): Result<IntendedActionsFrom, Error> => {
		if (intermediateActions.length === 0)
			return err(mustHaveAtLeastOneAction)

		return combine(
			intermediateActions.map(
				(i): Result<IntendedAction, Error> => {
					const intermediateActionType = i.type
					if (intermediateActionType === 'transfer') {
						if (isTransferTokensInput(i)) {
							return IntendedTransferTokens.create(i, from)
						} else {
							throw new Error('Not transfer tokens input')
						}
					} else if (intermediateActionType === 'stake') {
						if (isStakeTokensInput(i)) {
							return IntendedStakeTokens.create(i, from)
						} else {
							throw new Error('Not stake tokens input')
						}
					} else if (intermediateActionType === 'unstake') {
						return IntendedUnstakeTokens.create(
							i as UnstakeTokensInput,
							from,
						)
					} else {
						return err(
							new Error(
								'Incorrect implementation, forgot something...',
							),
						)
					}
				},
			),
		).map(intendedActions => ({ intendedActions, from }))
	}

	const syncBuildDoNotEncryptMessageIfAny = (
		from: AccountAddressT,
	): Result<TransactionIntent, Error> =>
		intendedActionsFromIntermediateActions(from).map(
			({ intendedActions }) => ({
				actions: intendedActions,
				message: maybePlaintextMsgToEncrypt
					.map(msg =>
						msg.plaintext
							? Message.createPlaintext(msg.plaintext).bytes
							: undefined,
					)
					.getOrUndefined(),
			}),
		)

	const build = (
		options: TransactionIntentBuilderOptions,
	): Observable<TransactionIntent> => {
		if (isTransactionIntentBuilderDoNotEncryptOption(options)) {
			if (
				maybePlaintextMsgToEncrypt.map(m => m.encrypt).getOrElse(false)
			) {
				const errMsg = `Message in transaction specifies it should be encrypted, but input to TransactionIntentBuilder build method specifies that it (the builder) should not encrypt the message, and does not provide any account with which we can perform encryption.`
				console.error(errMsg)
				log.error(errMsg)
				return throwError(new Error(errMsg))
			}

			return options.skipEncryptionOfMessageIfAny.spendingSender.pipe(
				mergeMap((from: AccountAddressT) =>
					toObservableFromResult(
						syncBuildDoNotEncryptMessageIfAny(from),
					),
				),
			)
		}

		if (!isTransactionIntentBuilderEncryptInput(options)) {
			throw new Error('Incorrect implementation')
		}

		const encryptingAccount$ = options.encryptMessageIfAnyWithAccount
		const spendingSender: Observable<AccountAddressT> =
			options.spendingSender ??
			options.encryptMessageIfAnyWithAccount.pipe(
				map(account => account.address),
			)
		return spendingSender.pipe(
			mergeMap((from: AccountAddressT) =>
				toObservableFromResult(
					intendedActionsFromIntermediateActions(from),
				),
			),
			mergeMap(
				(
					intendedActionsFrom: IntendedActionsFrom,
				): Observable<TransactionIntent> => {
					const transactionIntentWithoutEncryption = (
						plaintextMessage?: string,
					): Observable<TransactionIntent> => {
						log.info(
							`Successfully built transaction. Actions: ${intendedActionsFrom.intendedActions
								.map(action => action.type)
								.toString()}`,
						)
						return of({
							actions: intendedActionsFrom.intendedActions,
							message:
								plaintextMessage !== undefined
									? MessageEncryption.encodePlaintext(
											plaintextMessage,
									  )
									: undefined,
						})
					}

					return maybePlaintextMsgToEncrypt.match({
						Some: msgInTx => {
							if (!msgInTx.encrypt) {
								const errMsg =
									'You are trying to encrypt a message which was specified not to be encrypted.'
								console.error(errMsg)
								log.error(errMsg)
								return throwError(new Error(errMsg))
							}

							return encryptingAccount$.pipe(
								mergeMap(
									(
										encryptingAccount: AccountT,
									): Observable<ActorsInEncryption> =>
										ensureSingleRecipient({
											intendedActionsFrom,
											encryptingAccount,
										}),
								),
								mergeMap(
									(
										actors: ActorsInEncryption,
									): Observable<EncryptedMessageT> =>
										actors.encryptingAccount.encrypt({
											plaintext: msgInTx.plaintext,
											publicKeyOfOtherParty:
												actors.singleRecipientPublicKey,
										}),
								),
								map(
									(
										encryptedMessage: EncryptedMessageT,
									): TransactionIntent => {
										log.info(
											`Successfully built transaction with encrypted message. Actions: ${intendedActionsFrom.intendedActions
												.map(action => action.type)
												.toString()}`,
										)
										return {
											actions:
												intendedActionsFrom.intendedActions,
											message: encryptedMessage.combined(),
										}
									},
								),
							)
						},
						None: () =>
							transactionIntentWithoutEncryption(undefined),
					})
				},
			),
		)
	}

	const methods = {
		transferTokens,
		stakeTokens,
		unstakeTokens,
		build,
		message: replaceAnyPreviousMessageWithNew,
		__syncBuildDoNotEncryptMessageIfAny: syncBuildDoNotEncryptMessageIfAny,
	}

	return {
		...snapshotBuilderState(),
		...methods,
	}
}

export const TransactionIntentBuilder = {
	create,
}
