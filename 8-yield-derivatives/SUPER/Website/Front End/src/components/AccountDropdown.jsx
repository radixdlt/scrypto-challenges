import React, {useEffect} from 'react';
import PropTypes from "prop-types";
import {useAccount} from "../hooks/useAccount.jsx";


/**
 * AccountDropdown component that provides a dropdown menu for selecting an account.
 * It is designed to be used in parts of the application where users need to select an
 * account for various operations. The dropdown adapts its styling and behavior based
 * on the number of available accounts and whether the dropdown is enabled or disabled.
 *
 * @param {object} props - Component props
 * @param {string} props.selectedAccount - The currently selected account
 * @param {Function} props.setSelectedAccount - Function to update the selected account
 * @param {boolean} props.enableDropdown - Flag to enable or disable interaction with the dropdown
 * @returns {JSX.Element} The rendered AccountDropdown component.
 */
const AccountDropdown = (props) => {
    const { selectedAccount, setSelectedAccount, enableDropdown } = props;

    const { accounts } = useAccount();
    const [dropdownOpen, setDropdownOpen] = React.useState(false);
    const [selectStyle, setSelectStyle] = React.useState({
        width: "100%",
        fontSize: "1.15rem",
        backgroundColor: "var(--grey-2)",
        padding: "0.675rem 1rem",
        border: "1px solid var(--grey-5)",
        borderRadius: "8px",
        cursor: "pointer",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
    });
    const [active, setActive] = React.useState(false);

    useEffect(() => {
        // Automatically select the account if only one is available
        if (accounts.length === 1) {
            setSelectedAccount(accounts[0].address);
            handleSelectAccount(accounts[0])
        }
    }, [accounts, setSelectedAccount]);

    const toggleDropdown = () => {
        // Toggle the dropdown open/closed state
        setActive(!active);
        setDropdownOpen(!dropdownOpen);
    };

    // Handle selecting an account from the dropdown
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

    // Render a label for the account
    const renderAccountLabel = (account) => {
        const shortAddress = `${account.address.slice(0, 4)}...${account.address.slice(-6)}`;
        return `${account.label || "Account"} ${shortAddress}`;
    };

    return (
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
                disabled={!enableDropdown}
                style={selectStyle}
            >
                <span className="selected-value">
                    {!enableDropdown
                        ? "Setup Dev Mode to choose an account"
                        : selectedAccount && enableDropdown
                            ? renderAccountLabel(accounts.find((acc) => acc.address === selectedAccount))
                            : "Select an Account"}
                </span>
                <span className={selectedAccount ? "arrow-account" : "arrow"} />
            </button>

            {dropdownOpen && (
                <ul
                    className="select-dropdown"
                    role="listbox"
                    id="select-dropdown"
                >

                    {accounts.map((account) => (
                        <li
                            key={account.address}
                            role="option"
                            style={{
                                background: `var(--account-appearance-${account.appearanceId})`,
                            }}
                            onClick={() => handleSelectAccount(account)}
                        >
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
        </div>
    );
};
AccountDropdown.propTypes = {
    selectedAccount: PropTypes.string,
    setSelectedAccount: PropTypes.func,
    enableDropdown: PropTypes.bool,
};
export default AccountDropdown;
