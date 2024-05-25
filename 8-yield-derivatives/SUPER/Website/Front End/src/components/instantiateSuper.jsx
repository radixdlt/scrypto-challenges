import { useSendTransaction } from "../hooks/useSendTransaction";
import PropTypes from "prop-types";
import {newManifest} from "../manifests/newSuperManifest.js";
import {usePackageAddy, useDappDefinitionCaddy, useSaleLength} from "../hooks/useComponentDetails.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {useEffect, useState} from "react";
import {NewSaleDetailsToMongo} from "../api/posts.js";
import {useUpdateSaleDetails} from "../hooks/useUpdateSaleDetails.js";


InstantiateSuper.propTypes = {
    selectedAccount: PropTypes.string,
};

function InstantiateSuper(props) {

    const {selectedAccount} = props;

    const sendTransaction = useSendTransaction();
    const DappDefinition = useDappDefinitionCaddy();
    const PackageAddy = usePackageAddy();
    const testMode = useSaleLength();

    const [receipt, setReceipt] = useState(null);

    const handleStartSuper = async () => {
        if (!selectedAccount) {
            alert("Please select an account first.");
            return;
        }


        let manifest = newManifest(selectedAccount, DappDefinition, PackageAddy, testMode);

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
                // eslint-disable-next-line react-hooks/rules-of-hooks
                NewSaleDetailsToMongo(SaleDetailEvent).then()
            }
        }
    }, [receipt, SaleDetailEvent]); // This hook will re-run whenever receipt changes

    useUpdateSaleDetails()

    return (
        <div>
            <button
                id="owner-page-button"
                onClick={handleStartSuper}
                disabled={!selectedAccount}>
                Instantiate SUPER
            </button>
        </div>
    );
}

export default InstantiateSuper;
