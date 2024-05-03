import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import InstantiateSection from "../../sections/InstantiateSection.jsx";
import SaleStatusSection from "../../sections/SaleStatusSection.jsx";
import {useAccount} from "../../AccountContext.jsx";
import {useCallback, useEffect, useState} from "react";
import SelectOwnerSection from "../../sections/SelectOwnerSection.jsx";
function OwnerPage() {

    const { accounts } = useAccount();
    const [selectedAccount, setSelectedAccount] = useState(null);
    const [enableButtons, setEnableButtons] = useState(false);

    useEffect(() => {
        // Automatically enable buttons if accounts are available
        setEnableButtons(accounts.length > 0);
    }, [accounts]);

    const handleSelectAccount = useCallback((account) => {
        setSelectedAccount(account);
        // Example of additional logic that could be incorporated:
        // You might want to do more things here when an account is selected.
    }, [setSelectedAccount]);

    return (
        <>
            <PrimaryNavbar />

            <main>

                <SelectOwnerSection
                    selectedAccount={selectedAccount}
                    setSelectedAccount={setSelectedAccount}
                    enableButtons={enableButtons}
                />
                <InstantiateSection selectedAccount={selectedAccount}
                                    enableButtons={enableButtons}
                />
                <SaleStatusSection selectedAccount={selectedAccount}
                                   setSelectedAccount={handleSelectAccount}
                                   enableButtons={enableButtons}
                />

            </main>
        </>
    );
}

export default OwnerPage;