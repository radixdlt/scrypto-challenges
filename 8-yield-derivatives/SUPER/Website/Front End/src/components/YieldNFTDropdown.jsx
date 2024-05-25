import {useCallback, useEffect, useState} from 'react';
import PropTypes from 'prop-types';
import {useCombinedNftData} from "../hooks/useCombinedNftData.jsx";


const YieldNFTDropdown = ({ selectedAccount, enableSelectNft, YieldNftRaddy, setSelectedNft, setEnableInput }) => {

    const AccountNftsWithData = useCombinedNftData(YieldNftRaddy)

    if (AccountNftsWithData.length > 0 && enableSelectNft) {
        console.log("Account w/ nft data: ", AccountNftsWithData)
    }

    const [selectedAccountNfts, setSelectedAccountNfts] = useState([]);

    useEffect(() => {
        if (selectedAccount && AccountNftsWithData.length > 0) {

            const nftsForSelectedAccount = AccountNftsWithData.filter(address => address.address === selectedAccount);
            if (nftsForSelectedAccount) {
                setSelectedAccountNfts(nftsForSelectedAccount[0]); // Assuming you want the first match or handle multiple matches appropriately
                console.log("selectedAccountNfts",selectedAccountNfts);
            }
        } else {
            setSelectedAccountNfts([]); // Reset when selected account is not defined
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selectedAccount]);  // Ensure AccountNftsWithData is in the dependency array



    const [selectedNFT, setSelectedNFT] = useState(null);
    const [dropdownOpen, setDropdownOpen] = useState(false);
    const initialStyleState = {
        width: "100%",
        fontSize: "1.15rem",
        background: "var(--grey-2)",
        color: "white",
        padding: "0.675rem 1rem",
        border: "0.0625rem solid var(--grey-5)",
        borderRadius: "0.5rem",
        cursor: "pointer",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center"
    }
    const [selectStyle, setSelectStyle] = useState(initialStyleState);

    useEffect(() => {
        // Reset the selected NFT when selectedAccount changes
        setSelectedNFT(null);
        setDropdownOpen(false);
        setSelectStyle(initialStyleState);
        setSelectedNft(null);
        setEnableInput(false)
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selectedAccount]);

    const toggleDropdown = useCallback(() => {
        setDropdownOpen(prevOpen => !prevOpen);
    }, []);

    const handleSelectNFT = useCallback((nft, index) => {
        const bgSelector = index % 2 === 0 ? 'even' : 'odd';
        const fontSelector = index % 2 === 0 ? 'white' : 'var(--grey-1)';
        setSelectedNFT(nft);
        setSelectedNft(nft);
        setEnableInput(true);
        console.log("Selected NFT", nft);
        setSelectStyle({
            ...selectStyle,
            background: `var(--nft-appearance-${bgSelector}-bg)`,
            color: `${fontSelector}`,
            border: "none",
        });
        setDropdownOpen(false);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selectStyle]);

    const renderSelectedNFT = () => {
        if (selectedNFT) {
            return (
                <span className="nft-dropdown-option">
                    <span className="nft-dropdown-option-label">
                        NFT {selectedNFT.label}
                    </span>

                    <span className="nft-dropdown-option-data">
                        {`${selectedNFT.data.n_super_minted} SUPER Minted at Hour ${selectedNFT.data.hour_of_mint}`}
                    </span>
                </span>
            )
        } else {
            return "Select NFT ID";
        }
    };


        return (
            <div className={"custom-select" + (dropdownOpen ? " active" : "")}>

                <button
                    className="select-button"
                    role="combobox"
                    aria-haspopup="listbox"
                    aria-expanded={dropdownOpen}
                    disabled={!enableSelectNft}
                    onClick={toggleDropdown}
                    style={selectStyle}
                >
                    <span className="nft-dropdown-option">{renderSelectedNFT()}</span>
                    <span className="arrow" />
                </button>

                {dropdownOpen && (
                    <ul
                        className="select-dropdown"
                        role="listbox"
                        style={{ border: "0.0625rem solid var(--grey-5)", borderRadius: "0.5rem" }}
                    >

                        {selectedAccountNfts.nfts && selectedAccountNfts.nfts.length > 0 ? (
                            selectedAccountNfts.nfts.map((nft, index) => (
                                <li
                                    key={index}
                                    role="option"
                                    className={index % 2 === 0 ? 'nft-appearance-even' : 'nft-appearance-odd'}
                                    style={{
                                        padding: "0.5rem 0.625rem",
                                        cursor: "pointer"
                                    }}
                                    onClick={() => handleSelectNFT(nft, index)}
                                >
                                    {<span className="nft-dropdown-option">
                                        <span className="nft-dropdown-option-label">
                                            NFT {nft.label}
                                        </span>

                                        <span className="nft-dropdown-option-data">
                                            {`${nft.data.n_super_minted} SUPER Minted at Hour ${nft.data.hour_of_mint}`}
                                        </span>
                                    </span>}
                                </li>
                            ))
                        ) : (
                            <li style={{ padding: "1rem 2rem" }}>No NFTs found</li>
                        )}
                    </ul>
                )}
            </div>
        );

};

YieldNFTDropdown.propTypes = {
    selectedAccount: PropTypes.string,
    enableSelectNft: PropTypes.bool,
    YieldNftRaddy: PropTypes.string,
    setSelectedNft: PropTypes.func,
    setEnableInput: PropTypes.func,
};


export default YieldNFTDropdown;
