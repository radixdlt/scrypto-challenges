import PropTypes from 'prop-types';
import AccountDropdown from '../components/AccountDropdown.jsx';
import YieldNFTDropdown from '../components/YieldNFTDropdown.jsx';

const SelectNftSection = ({ selectedAccount, setSelectedAccount, enableButtons, enableSelectNft, YieldNftRaddy, setSelectedNft, setEnableInput }) => {
    return (
        <>
            <div className="choose-owner-container">

                <div className="choose-owner-heading-section">
                    <h2>Select Account and NFT</h2>
                    <p className="head-text">Select the account and the SUPER Yield NFT you want to manage.</p>
                </div>

                <div className="choose-owner-dropdown-container">

                    <AccountDropdown
                        selectedAccount={selectedAccount}
                        setSelectedAccount={setSelectedAccount}
                        enableDropdown={enableButtons} // Assuming this prop controls dropdown enable state
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
