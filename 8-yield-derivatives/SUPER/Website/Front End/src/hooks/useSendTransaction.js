import { useCallback } from "react";
import { useRdt } from "./useRdt";
import { useGatewayApi } from "./useGatewayApi";

export const useSendTransaction = () => {
  const rdt = useRdt();
  const gatewayApi = useGatewayApi();

  const sendTransaction = useCallback(
    // Send manifest to extension for signing
    async (transactionManifest, message) => {
        const transactionResult = await rdt.walletApi.sendTransaction({
            transactionManifest,
            version: 1,
            message,
        });

        if (transactionResult.isErr()) throw transactionResult.error;
        console.log("transaction result:", transactionResult.value.status);

        // Get the details of the transaction committed to the ledger
        const CommitedDetails = await gatewayApi.transaction.getCommittedDetails(
            transactionResult.value.transactionIntentHash, {receiptEvents: true}
        );

        const events = CommitedDetails.transaction.receipt.events;
        console.log("events:", events);

        return {transactionResult: transactionResult.value, events};
    },
    [gatewayApi, rdt]
  );

  return sendTransaction;
};
