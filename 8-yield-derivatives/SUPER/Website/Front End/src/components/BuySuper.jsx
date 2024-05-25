import { useSendTransaction } from "../hooks/useSendTransaction";
import PropTypes from "prop-types";
import {useEffect, useState} from "react";
import {useComponentAddy, useXrdAddy} from "../hooks/useComponentDetails.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {SendNewNftToMongo} from "../api/posts.js";


BuySuper.propTypes = {
    selectedAccount: PropTypes.string,
    enableButtons: PropTypes.bool,
    xrdAmount: PropTypes.string,
    error: PropTypes.string,
};

function BuySuper(props) {
    const [receipt, setReceipt] = useState(null);

    const {selectedAccount, enableButtons, xrdAmount, error} = props;

    const sendTransaction = useSendTransaction();

    const xrdAddy = useXrdAddy();
    const componentAddy = useComponentAddy();

    const handleBuySuper = async () => {

        if (!selectedAccount) {
            alert("Please select an account first.");
            return;
        }
        if (error) {
            alert("Fix the errors before submitting.");
            return;
        }

        const accountAddress = selectedAccount;

        let manifest = `
            CALL_METHOD
                Address("${accountAddress}")
                "withdraw"
                Address("${xrdAddy}")
                Decimal("${xrdAmount}");
            
            TAKE_FROM_WORKTOP
                Address("${xrdAddy}")
                Decimal("${xrdAmount}")
                Bucket("bucket1");
            
            CALL_METHOD
                Address("${componentAddy}")
                "deposit"
                Bucket("bucket1");
            
            CALL_METHOD
                Address("${accountAddress}")
                "deposit_batch"
                Expression("ENTIRE_WORKTOP");
            `;

        console.log("manifest", manifest);

        // eslint-disable-next-line no-unused-vars
        const { TxnResult, events } = await sendTransaction(manifest);
        await setReceipt(events);
    };

    const CreateYieldNFTEvent = useGetEventInReceipt(receipt, "CreateYieldNFTEvent");

    useEffect(() => {
        // Check if the receipt is not null and call the function
        if (receipt) {
            if(CreateYieldNFTEvent) {
                // Call the function when receipt is updated
                SendNewNftToMongo(CreateYieldNFTEvent)
            }
        }
    }, [receipt, CreateYieldNFTEvent]); // This hook will re-run whenever receipt changes

    return (
        <div>
            <button
                id="buy-super-button"
                onClick={handleBuySuper}
                disabled={!selectedAccount || !enableButtons}>
                Buy SUPER
            </button>
        </div>
    );
}

export default BuySuper;
