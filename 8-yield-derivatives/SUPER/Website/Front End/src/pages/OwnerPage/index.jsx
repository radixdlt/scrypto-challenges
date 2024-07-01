import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import InstantiateSection from "../../sections/InstantiateSection.jsx";
import SaleStatusSection from "../../sections/SaleStatusSection.jsx";
import {useEffect, useState} from "react";
import SelectOwnerSection from "../../sections/SelectOwnerSection.jsx";
import {useAccount} from "../../hooks/useAccount.jsx";


/**
 * OwnerPage component that combines various sections for the owner.
 *
 * @returns {JSX.Element} The rendered owner page component.
 */
function OwnerPage() {

    const { accounts } = useAccount();
    const [selectedAccount, setSelectedAccount] = useState(null);
    const [enableButtons, setEnableButtons] = useState(true);

    useEffect(() => {
        // Automatically enable buttons if accounts are available
        setEnableButtons(accounts.length > 0);
    }, [accounts]);

    return (
        <>
            <PrimaryNavbar />

            <main>

                {/* Section to select the owner account */}
                <SelectOwnerSection
                    selectedAccount={selectedAccount}
                    setSelectedAccount={setSelectedAccount}
                    enableButtons={enableButtons}
                />

                {/* Section to instantiate an instance of the DApp */}
                <InstantiateSection selectedAccount={selectedAccount}
                                    enableButtons={enableButtons}
                />

                {/* Section to display and manage sale status */}
                <SaleStatusSection selectedAccount={selectedAccount}
                                   enableButtons={enableButtons}
                />

            </main>
        </>
    );
}

export default OwnerPage;