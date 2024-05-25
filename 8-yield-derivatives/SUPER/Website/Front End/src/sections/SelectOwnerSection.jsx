import AccountDropdown from "../components/AccountDropdown.jsx";
import PropTypes from "prop-types";  // Ensure the path is correct

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
                        enableButtons={enableButtons}
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
