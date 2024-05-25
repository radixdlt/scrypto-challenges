import { useState, useEffect } from "react";
import BuySuper from "../components/BuySuper.jsx";
import { } from "react";
import {useAccount} from "../hooks/useAccount.jsx";

const DevSection = () => {

    const { accounts, selectedAccount, setSelectedAccount } = useAccount();
    const [dropdownOpen, setDropdownOpen] = useState(false);
    const [enableButtons, setEnableButtons] = useState(false);
    const [xrdAmount, setXrdAmount] = useState('');
    const [error, setError] = useState('');
    const [selectStyle, setSelectStyle] = useState({
        width: "100%",
        fontSize: "1.15rem",
        backgroundColor: "var(--grey-2)",
        padding: "0.675em 1em",
        border: "1px solid var(--grey-5)",
        borderRadius: "8px",
        cursor: "pointer",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
    });
    const [active, setActive] = useState(false);

    useEffect(() => {
        if (accounts.length > 0) {
            setEnableButtons(true);
        } else {
            setEnableButtons(false);
        }
    }, [accounts]); // Only re-run the effect if count changes
    const isNumeric = num => !isNaN(num);
    const handleChange = (e) => {
        const val = e.target.value.trim();
        if (val === '' || (isNumeric(val))) {
            setXrdAmount(val);
            setError('');
        } else {
            setError('Please enter a numeric value.');
        }
    };
    const toggleDropdown = () => {
        setActive(!active);
        setDropdownOpen(!dropdownOpen);
    };

    const handleSelectAccount = (account) => {
        setSelectedAccount(account.address);
        setSelectStyle({
            ...selectStyle,
            background: `var(--account-appearance-${account.appearanceId})`,
            border: "none",
        });
        setActive(false);
        setDropdownOpen(false);
    };

    const renderAccountLabel = (account) => {
        const shortAddress = `${account.address.slice(
            0,
            4
        )}...${account.address.slice(-6)}`;
        return `${account.label || "Account"} ${shortAddress}`;
    };

    return (
        <>
            <div className="heading-section">
                <h2>Developer Page</h2>
                <p className="head-text">
                    You must have an <span className="hello-token-pink">Owner Badge</span> to interact with this page.
                </p>
            </div>


            <div className="buy-super-container">
                <div className="buy-super-left-col">
                    <h3>Have you Setup Dev Mode?</h3>

                    {/* <!-- ************ Custom Select ****************** --> */}

                    <>

                        <div style={{ display: "inline-flex", justifyContent: "flex-start", width: "60vw" }}>

                            <div className={"custom-select" + (active ? " active" : "")}>
                                <button
                                    className={
                                        selectedAccount ? "select-button-account" : "select-button"
                                    }
                                    role="combobox"
                                    aria-haspopup="listbox"
                                    aria-expanded={dropdownOpen}
                                    onClick={toggleDropdown}
                                    aria-controls="select-dropdown"
                                    disabled={!enableButtons}
                                    style={selectStyle}>
                <span className="selected-value">
                  {!enableButtons
                      ? "Setup Dev Mode to choose an account"
                      : selectedAccount && enableButtons
                          ? renderAccountLabel(
                              accounts.find((acc) => acc.address === selectedAccount)
                          )
                          : "Select an Account"}
                </span>
                                    <span className={selectedAccount ? "arrow-account" : "arrow"}/>
                                </button>

                                { dropdownOpen && (
                                    <ul
                                        className="select-dropdown"
                                        role="listbox"
                                        id="select-dropdown">
                                        {accounts.map((account) => (
                                            <li
                                                key={account.address}
                                                role="option"
                                                style={{
                                                    background: `var(--account-appearance-${account.appearanceId})`,
                                                }}
                                                onClick={() => handleSelectAccount(account)}>
                                                <label>{renderAccountLabel(account)}</label>
                                                <input
                                                    type="radio"
                                                    name={account.label}
                                                    value={account.address}
                                                    defaultChecked={selectedAccount === account.address}
                                                />
                                            </li>
                                        ))}
                                    </ul>
                                )}

                                {error && <div style={{color: 'red'}}>{error}</div>}

                            </div>

                            <span style={{display: 'inline-flex', marginLeft: '0.25rem'}}>

                <input
                    type={"text"}
                    id={"buy-super-input"}
                    onChange={handleChange}
                    value={xrdAmount}
                    placeholder="Enter XRD Amount"
                    style={{marginBottom: '0.625rem', width: '15vw', marginLeft: '0.625rem'}}
                />

                <p id={'input-suffix'}>
                  XRD
                </p>

              </span>

                        </div>

                        <BuySuper
                            selectedAccount={selectedAccount}
                            enableButtons={enableButtons}
                            xrdAmount={xrdAmount}
                            error={error}
                        />

                    </>
                </div>
                <div className="super_s">
                    <img src="src/assets/logo/transparent/white/super_s.svg" alt="super logo" />
                </div>
            </div>
        </>
    );
};

export default DevSection;
