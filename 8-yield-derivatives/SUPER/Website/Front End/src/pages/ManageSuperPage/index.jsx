import {/*useCallback,*/ useEffect, useState} from "react";
import AccountSelectSection from "../../sections/AccountSelectSection.jsx";
import MemoizedSplitNftSection from "../../sections/SplitNftSection.jsx";
import {useAccount} from "../../hooks/useAccount.jsx";
import {useYieldNftRaddy} from "../../hooks/useComponentDetails.js";
//import SelectNFTSection from "../../sections/SelectNFTSection.jsx";


function ManageSuperPage() {

    const { accounts } = useAccount();
    const YieldNftRaddy = useYieldNftRaddy();
    const [selectedAccount, setSelectedAccount] = useState(null);
    const [enableButtons, setEnableButtons] = useState(false);
    const [enableSelectNft, setEnableSelectNft] = useState(false);


    useEffect(() => {
        // Automatically enable buttons if accounts are available
        setEnableButtons(accounts.length > 0);
        setEnableSelectNft(accounts.length > 0);

    }, [accounts]);

    return (
        <>
            <main>


                <AccountSelectSection
                    selectedAccount={selectedAccount}
                    setSelectedAccount={setSelectedAccount}
                    enableButtons={enableButtons}
                />

                <MemoizedSplitNftSection
                selectedAccount={selectedAccount}
                enableSelectNft={enableSelectNft}
                YieldNftRaddy = {YieldNftRaddy}
                />

            </main>
        </>
    );
}

export default ManageSuperPage;