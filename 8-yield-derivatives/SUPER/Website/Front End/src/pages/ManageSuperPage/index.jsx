import SelectNftSection from "../../sections/SelectNFTSection.jsx";
import {useEffect, useState} from "react";
import {useAccount} from "../../hooks/useAccount.jsx";
import {useYieldNftRaddy} from "../../hooks/useComponentDetails.js";
import SplitNFTSectionV2 from "../../sections/SplitNFTSectionV2.jsx";

/**
 * ManageSuperPage component that serves as a page for managing the SUPER yield NFT.
 *
 * @returns {JSX.Element} The rendered "Manage Super" page component.
 */
const ManageSuperPage = () => {
    const { accounts } = useAccount();
    const [selectedAccount, setSelectedAccount] = useState(null); // State to manage the selected account
    const [enableSelectNft, setEnableSelectNft] = useState(true); // State to enable/disable NFT selection
    const [selectedNft, setSelectedNft] = useState(null); // State to manage the selected NFT
    const [enableInput, setEnableInput] = useState(false); // State to enable/disable inputs
    const [enableButtons, setEnableButtons] = useState(false); // State to enable/disable buttons
    const YieldNftRaddy = useYieldNftRaddy(); // Fetch the resource address of the yield NFT


    useEffect(() => {
        // Automatically enable buttons if accounts are available
        setEnableButtons(accounts.length > 0);
        setEnableSelectNft(accounts.length > 0);

    }, [accounts]);

    useEffect(() => {
        // Enable input fields if both an account and an NFT are selected
        if (selectedNft && selectedAccount) {
                setEnableInput(true)
            } else {
                setEnableInput(false)
            }
        }, [selectedNft, selectedAccount]);

    return (
        <div className="manage-super-page">

            <h1>Manage Your Super Assets</h1>

            <SelectNftSection
                selectedAccount={selectedAccount}
                setSelectedAccount={setSelectedAccount}
                enableButtons={enableButtons}
                enableSelectNft={enableSelectNft}
                YieldNftRaddy={YieldNftRaddy}
                setSelectedNft={setSelectedNft}
                setEnableInput={setEnableInput}
            />

            <SplitNFTSectionV2
                selectedAccount={selectedAccount}
                selectedNft={selectedNft}
                YieldNftRaddy={YieldNftRaddy}
                enableInput = {enableInput}
            />

        </div>
    );
};

export default ManageSuperPage;