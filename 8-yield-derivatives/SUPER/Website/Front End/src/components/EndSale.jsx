import { useSendTransaction } from "../hooks/useSendTransaction.js";
import PropTypes from "prop-types";
import {useEffect, useState} from "react";
import {useComponentAddy, useOwnerBadgeRaddy} from "../hooks/useComponentDetails.js";
import {endSaleManifest} from "../manifests/endSaleManifest.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import { UpdateSaleDetailsToMongo} from "../api/posts.js";
import {useUpdateSaleDetails} from "../hooks/useUpdateSaleDetails.js";


EndSale.propTypes = {
    selectedAccount: PropTypes.string,
};

/**
 * EndSale component provides functionality for ending a sale.
 * It includes a button to end the sale, and upon clicking the button,
 * it constructs a transaction manifest and sends it using the sendTransaction function.
 * It also handles the validation to ensure a user account is selected before attempting the transaction.
 * Upon successful transaction, it logs the transaction events and updates sale details in MongoDB.
 *
 * @param {object} props - Component props
 * @param {string} props.selectedAccount - The currently selected account
 * @returns {JSX.Element} The rendered EndSale component.
 */
function EndSale(props) {

    const {selectedAccount} = props;
    const sendTransaction = useSendTransaction(); // Hook to send transaction manifests
    const componentAddress = useComponentAddy(); // Hook to get the component address
    const ownerBadgeAddress = useOwnerBadgeRaddy(); // Hook to get the owner badge resource address

    const [receipt, setReceipt] = useState(null); // State to store the transaction receipt

    /**
     * Handles the action to end the sale by constructing and sending the transaction manifest.
     */
    const handleStartSuper = async () => {
        if (!selectedAccount) {
            alert("Please select an account first.");
            return;
        }

        const manifest = endSaleManifest(selectedAccount, componentAddress, ownerBadgeAddress);

        console.log("manifest", manifest);

        // eslint-disable-next-line no-unused-vars
        const { TxnResult, events } = await sendTransaction(manifest);

        await setReceipt(events);

    };

    const SaleDetailEvent = useGetEventInReceipt(receipt, "SaleDetailEvent");

    useEffect(() => {
        // Check if receipt is not null and call the function
        if (receipt) {
            if(SaleDetailEvent) {
                // Call the function when receipt is updated
                UpdateSaleDetailsToMongo(SaleDetailEvent)
            }
        }
    }, [receipt, SaleDetailEvent]); // This hook will re-run whenever receipt changes

    useUpdateSaleDetails(); // Hook to update sale details

    return (
        <div>
            <button
                id="sale-status-button"
                onClick={handleStartSuper}
                disabled={!selectedAccount}>
                End Sale
            </button>
        </div>
    );
}

export default EndSale;
