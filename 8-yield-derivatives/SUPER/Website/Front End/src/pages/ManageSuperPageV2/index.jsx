import SelectNftSection from "../../sections/SelectNFTSection.jsx";
import {useEffect, useState} from "react";
import {useAccount} from "../../hooks/useAccount.jsx";
import {useYieldNftRaddy} from "../../hooks/useComponentDetails.js";

const ManageSuperPage = () => {
    const { accounts } = useAccount();
    // State to manage the selected account and NFT-related options
    const [selectedAccount, setSelectedAccount] = useState(null);
    // eslint-disable-next-line no-unused-vars
    const [enableSelectNft, setEnableSelectNft] = useState(true); // Assuming this might be toggled based on some conditions
    // eslint-disable-next-line no-unused-vars
    const [selectedNft, setSelectedNft] = useState(null);
    // eslint-disable-next-line no-unused-vars
    const [enableInput, setEnableInput] = useState(false);
    const [enableButtons, setEnableButtons] = useState(false);
    const YieldNftRaddy = useYieldNftRaddy();

    useEffect(() => {
        // Automatically enable buttons if accounts are available
        setEnableButtons(accounts.length > 0);
        setEnableSelectNft(accounts.length > 0);

    }, [accounts]);
    // You might also manage other states related to specific functionalities on this page

    return (
        <div className="manage-super-page">
            <h1>Manage Your Super Assets</h1>
            <SelectNftSection
                selectedAccount={selectedAccount}
                setSelectedAccount={setSelectedAccount}
                enableButtons={enableButtons}
                enableSelectNft={enableSelectNft}
                YieldNftRaddy={YieldNftRaddy} // This would be dynamically fetched or set based on your application's logic
                setSelectedNft={setSelectedNft}
                setEnableInput={setEnableInput}
            />
            {/* Other sections or components related to managing super assets could also be added here */}
        </div>
    );
};

export default ManageSuperPage;