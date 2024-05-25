import { useSendTransaction } from "../hooks/useSendTransaction";
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

function SplitNftButton(props) {

    const [receipt, setReceipt] = useState(null);
    const {selectedAccount, enableButton, YieldNftRaddy, selectedNft, numSplits} = props;
    const sendTransaction = useSendTransaction();
    const componentAddy = useComponentAddy();

    const handleBuySuper = async () => {

        if (!selectedAccount || !enableButton) {
            alert("Please select an account first.");
            return;
        }

        let manifest = splitNftManifest(selectedAccount, componentAddy, YieldNftRaddy, selectedNft.value, numSplits);

        console.log("manifest", manifest);

        // eslint-disable-next-line no-unused-vars
        const { TxnResult, events } = await sendTransaction(manifest);
        await setReceipt(events);
    };

    const SplitNFTEvent = useGetEventInReceipt(receipt, "CreateYieldNFTEvent");

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
