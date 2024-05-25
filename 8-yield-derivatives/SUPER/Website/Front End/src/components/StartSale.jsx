import { useSendTransaction } from "../hooks/useSendTransaction";
import PropTypes from "prop-types";
import {useEffect, useState} from "react";
import {useComponentAddy, useOwnerBadgeRaddy, useXrdAddy} from "../hooks/useComponentDetails.js";
import {startSaleManifest} from "../manifests/startSaleManifest.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {UpdateSaleDetailsToMongo} from "../api/posts.js";
import {useUpdateSaleDetails} from "../hooks/useUpdateSaleDetails.js";


StartSale.propTypes = {
    selectedAccount: PropTypes.string,
};

function StartSale(props) {

    const {selectedAccount} = props;

    const sendTransaction = useSendTransaction();
    const xrdAddy = useXrdAddy();
    const componentAddress = useComponentAddy();
    const ownerBadgeAddress = useOwnerBadgeRaddy();

    const [receipt, setReceipt] = useState(null);

    const handleStartSuper = async () => {
        if (!selectedAccount) {
            alert("Please select an account first.");
            return;
        }

        const manifest = startSaleManifest(selectedAccount, componentAddress, xrdAddy, ownerBadgeAddress);

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

    useUpdateSaleDetails()

    return (
        <div>
            <button
                id="sale-status-button"
                onClick={handleStartSuper}
                disabled={!selectedAccount}>
                Start Sale
            </button>
        </div>
    );
}

export default StartSale;
