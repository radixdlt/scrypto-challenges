import { useSendTransaction } from "../hooks/useSendTransaction.js";
import PropTypes from "prop-types";
import {useEffect, useState} from "react";
import {useComponentAddy, useXrdAddy} from "../hooks/useComponentDetails.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {SendNewNftToMongo} from "../api/posts.js";
import {buyManifest} from "../manifests/buyManifest.js";


BuySuper.propTypes = {
    selectedAccount: PropTypes.string,
    enableButtons: PropTypes.bool,
    xrdAmount: PropTypes.string,
    error: PropTypes.string,
};

/**
 * BuySuper component that provides functionality for purchasing SUPER tokens.
 * It includes a button to initiate the purchase, which triggers a transaction using
 * the Radix dApp Toolkit.
 * Upon successful transaction, it logs the transaction events
 * and sends new NFT data to MongoDB.
 *
 * @param {object} props - Component props
 * @param {string} props.selectedAccount - The currently selected account
 * @param {boolean} props.enableButtons - Flag to enable or disable the button
 * @param {string} props.xrdAmount - The amount of XRD to be used in the transaction
 * @param {string} props.error - Error message to display if there's an issue with the input
 * @returns {JSX.Element} The rendered BuySuper component.
 */
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
        let manifest = buyManifest(accountAddress, componentAddy, xrdAddy, xrdAmount)

        console.log("manifest", manifest);

        // Send the transaction and get the result and events
        // eslint-disable-next-line no-unused-vars
        const { TxnResult, events } = await sendTransaction(manifest);
        await setReceipt(events);
    };

    // Extract the CreateYieldNFTEvent from the receipt
    const CreateYieldNFTEvent = useGetEventInReceipt(receipt, "CreateYieldNFTEvent");

    useEffect(() => {
        // Check if the receipt is not null and call the function
        if (receipt) {
            if(CreateYieldNFTEvent) {
                // Send the new NFT data to MongoDB
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
