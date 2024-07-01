import { useSendTransaction } from "../hooks/useSendTransaction.js";
import PropTypes from "prop-types";
import {useEffect, useState} from "react";
import {useComponentAddy} from "../hooks/useComponentDetails.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {SendNewNftToMongo} from "../api/posts.js";
import {splitNftManifest} from "../manifests/splitNftManifest.js";


SplitNftButton.propTypes = {
    selectedAccount: PropTypes.string,
    enableButton: PropTypes.bool,
    YieldNftRaddy: PropTypes.string,
    selectedNft: PropTypes.object,
    numSplits: PropTypes.string,
};

/**
 * SplitNftButton component provides functionality for splitting a SUPER Yield NFT into multiple NFTs.
 * It includes a button to initiate the split process.
 * Upon clicking the button, it constructs a transaction
 * manifest and sends it using the sendTransaction function.
 * It also handles the validation to ensure
 * a user account is selected before attempting the transaction.
 * Upon successful transaction, it logs the
 * transaction events and sends new NFT data to MongoDB.
 *
 * @param {object} props - Component props
 * @param {string} props.selectedAccount - The currently selected account
 * @param {boolean} props.enableButton - Flag to enable or disable the split button
 * @param {string} props.YieldNftRaddy - The resource address of the yield NFT
 * @param {object} props.selectedNft - The selected NFT object
 * @param {string} props.numSplits - The number of splits to be made
 * @returns {JSX.Element} The rendered SplitNftButton component.
 */
function SplitNftButton(props) {

    const [receipt, setReceipt] = useState(null); // State to store the transaction receipt
    const { selectedAccount, enableButton, YieldNftRaddy, selectedNft, numSplits } = props;
    const sendTransaction = useSendTransaction(); // Hook to send transaction manifests
    const componentAddy = useComponentAddy(); // Hook to get the component address

    // Function to handle the split NFT action
    const handleBuySuper = async () => {
        if (!selectedAccount || !enableButton) {
            alert("Please select an account first.");
            return;
        }

        // Construct the transaction manifest
        let manifest = splitNftManifest(selectedAccount, componentAddy, YieldNftRaddy, selectedNft.value, numSplits);

        console.log("manifest", manifest);

        // Send the transaction and get the result
        // eslint-disable-next-line no-unused-vars
        const { TxnResult, events } = await sendTransaction(manifest);
        await setReceipt(events);
    };

    const SplitNFTEvent = useGetEventInReceipt(receipt, "SplitNFTEvent");

    useEffect(() => {
        // Check if the receipt is not null and call the function
        if (receipt) {
            if(SplitNFTEvent) {
                // Call the function when receipt is updated
                SendNewNftToMongo(SplitNFTEvent)
            }
        }
    }, [receipt, SplitNFTEvent]); // This hook will re-run whenever receipt changes

    return (
        <div>
            <button
                id="buy-super-button"
                onClick={handleBuySuper}
                disabled={!selectedAccount || !enableButton}>
                Split NFT
            </button>
        </div>
    );
}

export default SplitNftButton;
