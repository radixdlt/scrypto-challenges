import {
  GatewayApiClient,
  TransactionPreviewRequest,
  TransactionStatus,
} from '@radixdlt/radix-dapp-toolkit'
import { ResultAsync, err, errAsync, okAsync } from 'neverthrow'
import { typedError } from './helpers/typed-error'
import { config } from './config'
import { filter, first, firstValueFrom, switchMap } from 'rxjs'
import { logger } from './helpers/logger'
import { ExponentialBackoff } from './helpers/exponential-backoff'

export type GatewayClient = ReturnType<typeof GatewayClient>
const GatewayClient = (basePath: string) => {
  const { status, transaction, state } = GatewayApiClient.initialize({
    basePath,
    applicationName: 'TS wallet',
  })

  const getEpoch = () =>
    ResultAsync.fromPromise(status.getCurrent(), typedError).map(
      (response) => response.ledger_state.epoch
    )

  const submitNotarizedTransactionHex = (notarized_transaction_hex: string) =>
    ResultAsync.fromPromise(
      transaction.innerClient.transactionSubmit({
        transactionSubmitRequest: {
          notarized_transaction_hex,
        },
      }),
      typedError
    )

  const getTransactionStatus = (txId: string) =>
    ResultAsync.fromPromise(transaction.getStatus(txId), typedError)

  const getCommittedDetails = (txId: string) =>
    ResultAsync.fromPromise(
      transaction.getCommittedDetails(txId),
      typedError
    ).map((res) => ({
      epoch: res.transaction.epoch,
      round: res.transaction.round,
      status: res.transaction.transaction_status,
      date: res.transaction.confirmed_at,
      fee: res.transaction.fee_paid,
      message: (res.transaction.message as any)?.content?.value,
      encodedManifest: res.transaction.raw_hex,
      receipt: res.transaction.receipt,
      events: res.transaction.receipt?.events,
      affectedEntities: res.transaction.affected_global_entities || [],
      createdEntities:
        ((res.transaction.receipt?.state_updates as any)
          ?.new_global_entities as any[]) || [],
      stateVersion: res.transaction.state_version,
    }))

  const getState = (addresses: string[]) =>
    ResultAsync.fromPromise(
      state.innerClient.stateEntityDetails({
        stateEntityDetailsRequest: { addresses },
      }),
      typedError
    )

  const preview = (transactionPreviewRequest: TransactionPreviewRequest) =>
    ResultAsync.fromPromise(
      transaction.innerClient.transactionPreview({
        transactionPreviewRequest,
      }),
      typedError
    )

  const pollTransactionStatus = (txId: string) => {
    const retry = ExponentialBackoff({
      maxDelayTime: 5_000,
      multiplier: 2,
      timeout: 60_000,
      interval: 1_000,
    })

    const completedTransactionStatus = new Set<TransactionStatus>([
      'CommittedSuccess',
      'CommittedFailure',
      'Rejected',
    ])

    return ResultAsync.fromPromise(
      firstValueFrom(
        retry.withBackoff$.pipe(
          switchMap((result) => {
            if (result.isErr()) return [err(result.error)]

            return getTransactionStatus(txId).andThen((response) => {
              logger?.debug({
                event: 'pollTransactionStatus',
                retry: result.value + 1,
                status: response.status,
                txId,
              })

              if (completedTransactionStatus.has(response.status)) {
                return response.status === 'CommittedSuccess'
                  ? okAsync(response)
                  : errAsync(response)
              }

              retry.trigger.next()
              return okAsync(undefined)
            })
          }),
          filter(
            (result) =>
              (result && result.isOk() && !!result.value) || result.isErr()
          ),
          first()
        )
      ),
      typedError
    ).andThen((result) => result)
  }

  return {
    getCommittedDetails,
    pollTransactionStatus,
    getEpoch,
    submitNotarizedTransactionHex,
    getTransactionStatus,
    getState,
    preview,
  }
}
export const gatewayClient = GatewayClient(config.network.gatewayUrl)
