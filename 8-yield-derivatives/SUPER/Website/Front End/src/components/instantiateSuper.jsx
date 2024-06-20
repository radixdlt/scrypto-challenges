import { useSendTransaction } from "../hooks/useSendTransaction.js";
import PropTypes from "prop-types";
import {newManifest} from "../manifests/newSuperManifest.js";
import {usePackageAddy, useDappDefinitionCaddy} from "../hooks/useComponentDetails.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {useEffect, useState} from "react";
import {NewSaleDetailsToMongo} from "../api/posts.js";
import {useUpdateSaleDetails} from "../hooks/useUpdateSaleDetails.js";


InstantiateSuper.propTypes = {
    selectedAccount: PropTypes.string,
};

/**
 * InstantiateSuper component provides functionality for instantiating a new SUPER component.
 * It includes a button to initiate the instantiation, and upon clicking the button,
 * it constructs a transaction manifest and sends it using the sendTransaction function.
 * It also handles the validation to ensure a user account is selected before attempting the transaction.
 * Upon successful transaction, it logs the transaction events and updates sale details in MongoDB.
 *
 * @param {object} props - Component props
 * @param {string} props.selectedAccount - The currently selected account
 * @returns {JSX.Element} The rendered InstantiateSuper component.
 */
function InstantiateSuper(props) {

    const {selectedAccount} = props;
    const sendTransaction = useSendTransaction(); // Hook to send transaction manifests
    const DappDefinition = useDappDefinitionCaddy(); // Hook to get the dApp definition address
    const PackageAddy = usePackageAddy(); // Hook to get the package address

    const [receipt, setReceipt] = useState(null); // State to store the transaction receipt

    /**
     * Handles the action to instantiate the SUPER component by constructing and sending the transaction manifest.
     */
    const handleStartSuper = async () => {
        if (!selectedAccount) {
            alert("Please select an account first.");
            return;
        }


        let manifest = newManifest(selectedAccount, DappDefinition, PackageAddy);

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

    useUpdateSaleDetails(); // Hook to update sale details

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
