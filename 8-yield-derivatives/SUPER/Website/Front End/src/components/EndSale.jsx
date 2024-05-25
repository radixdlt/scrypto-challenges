import { useSendTransaction } from "../hooks/useSendTransaction";
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

function EndSale(props) {

    const {selectedAccount} = props;

    const sendTransaction = useSendTransaction();
    const componentAddress = useComponentAddy();
    const ownerBadgeAddress = useOwnerBadgeRaddy();

    const [receipt, setReceipt] = useState(null);

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

    useUpdateSaleDetails()

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
