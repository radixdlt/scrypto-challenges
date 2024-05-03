import { transactionApi } from './index';

// ************ Fetch the transaction status from the Gateway API ************
export async function getTransactionStatus(transactionIntentHash: string) {
  try {
    return await transactionApi.transactionStatus({
      transactionStatusRequest: {
        intent_hash: transactionIntentHash,
      },
    });
  } catch (error) {
    console.error('Error getting transaction status:', error);
    throw error;
  }
}
