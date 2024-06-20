import PropTypes from 'prop-types';
import AccountDropdown from '../components/AccountDropdown.jsx';
import YieldNFTDropdown from '../components/YieldNFTDropdown.jsx';

/**
 * SelectNftSection allows users to select an account and the SUPER Yield NFT they want to manage.
 * This section is commonly used in parts of the application that require the user to operate
 * under an account with they Yield  NFTs.
 *
 * @param {object} props - Component props
 * @param {string} props.selectedAccount - Currently selected account
 * @param {Function} props.setSelectedAccount - Function to update the selected account
 * @param {boolean} props.enableButtons - Flag to enable interaction with the account dropdown
 * @param {boolean} props.enableSelectNft - Flag to enable interaction with the NFT dropdown
 * @param {string} props.YieldNftRaddy - The resource address of the yield NFT
 * @param {Function} props.setSelectedNft - Function to update the selected NFT
 * @param {Function} props.setEnableInput - Function to enable or disable input fields based on the selected account and NFT
 * @returns {JSX.Element} The rendered "Select Account and NFT" section component.
 */
const SelectNftSection = ({ selectedAccount, setSelectedAccount, enableButtons, enableSelectNft, YieldNftRaddy, setSelectedNft, setEnableInput }) => {
    return (
        <>
            <div className="choose-owner-container">

                <div className="choose-owner-heading-section">
                    <p className="head-text">Select the account and the SUPER Yield NFT you want to manage.</p>
                </div>

                <div className="choose-owner-dropdown-container">

                    <AccountDropdown
                        selectedAccount={selectedAccount}
                        setSelectedAccount={setSelectedAccount}
                        enableDropdown={enableButtons}
                    />
                </div>

                <div className="select-nft-dropdown-container">

                <YieldNFTDropdown
                        selectedAccount={selectedAccount}
                        enableSelectNft={enableSelectNft} // This might control the dropdown enable state based on NFT specifics
                        YieldNftRaddy={YieldNftRaddy}
                        setSelectedNft={setSelectedNft}
                        setEnableInput={setEnableInput}
                    />

                </div>

                <div className="manage-button-container">
                    
                </div>

            </div>
        </>
    );
};

SelectNftSection.propTypes = {
    selectedAccount: PropTypes.string,
    setSelectedAccount: PropTypes.func.isRequired,
    enableButtons: PropTypes.bool,
    enableSelectNft: PropTypes.bool,
    YieldNftRaddy: PropTypes.string,
    setSelectedNft: PropTypes.func.isRequired,
    setEnableInput: PropTypes.func.isRequired
};

export default SelectNftSection;
