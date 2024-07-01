import AccountDropdown from "../components/AccountDropdown.jsx";
import PropTypes from "prop-types";  // Ensure the path is correct

/**
 * SelectOwnerSection allows users to select an account that holds the SUPER owner badge.
 * This section is commonly used in parts of the application that require the user to operate
 * under an account with specific permissions or roles.
 *
 * @param {object} props - Component props
 * @param {string} props.selectedAccount - Currently selected account
 * @param {Function} props.setSelectedAccount - Function to update the selected account
 * @param {boolean} props.enableButtons - Flag to enable interaction with the dropdown
 * @returns {JSX.Element} The rendered "Select Owner" section component.
 */
const SelectOwnerSection = (props) => {
    const { selectedAccount, setSelectedAccount, enableButtons } = props;

    return (
        <>

            <div className="choose-owner-container" > {/* Using existing CSS class names for styling */}
                <div className="choose-owner-heading-section">
                    <h2>Select Owner</h2>
                    <p className="head-text">
                        Select the account with <span className="hello-token-pink">SUPER</span> owner badge.
                    </p>
                </div>
                <div className="choose-owner-dropdown-container"> {/* Again, reusing the same styling */}
                    <AccountDropdown
                        selectedAccount={selectedAccount}
                        setSelectedAccount={setSelectedAccount}
                        enableDropdown={enableButtons}
                    />
                </div>
            </div>
        </>
    );
};

SelectOwnerSection.propTypes = {
    selectedAccount: PropTypes.string,
    setSelectedAccount: PropTypes.func,
    enableButtons: PropTypes.bool,
};

export default SelectOwnerSection;
