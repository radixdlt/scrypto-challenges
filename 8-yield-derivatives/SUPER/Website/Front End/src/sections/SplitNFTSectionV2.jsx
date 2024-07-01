import React, {useEffect, useState} from "react";
import PropTypes from "prop-types";
import SplitNftButton from "../components/SplitNftButton.jsx";


/**
 * Handles the logic for splitting a selected NFT into multiple parts.
 * Users can specify the number of parts to split the NFT into, given that
 * the input is a numeric value and does not exceed 50.
 *
 * @param {string} selectedAccount The current user's account address.
 * @param {object} selectedNft Details of the selected NFT.
 * @param {string} YieldNftRaddy The resource address of the NFT.
 * @param {boolean} enableInput Controls if the input field is enabled.
 */
const SplitNftSectionV2 = ({ selectedAccount, selectedNft, YieldNftRaddy, enableInput }) => {

    const [numSplits, setNumSplits] = useState(0); // Tracks the number of splits specified by the user.
    const [error, setError] = useState(''); // Stores error messages related to user input.
    const [enableButton, setEnableButton] = useState(false); // Controls the activation state of the split button.
    const [input, setInput] = useState(""); // Holds the current value of the input field.
    const [nftLabel, setNftLabel] = useState(""); // Displays the label of the selected NFT.

    // Updates the NFT label whenever the selected NFT changes.
    useEffect(() => {
        if (selectedNft) {
            setNftLabel(selectedNft.label)
        }
        else {
            setNftLabel("")
        }
    }, [selectedNft]);

    // Utilities to check inputted values
    const isNumeric = num => !isNaN(num);
    const isInteger = num => Number.isInteger(num);
    const isLowerThan50 = num => num <= 50;

    // Validates the user input and updates relevant states accordingly.
    useEffect(() => {
        if (input !== "") {
            const val = parseFloat(input);
            if (val === '' || (isNumeric(val) && isInteger(val) && isLowerThan50(val))) {
                setNumSplits(val);
                setEnableButton(true);
                setError('');
            } else {
                setError('Please enter a integer value (Max. 50).');
                setEnableButton(false);
            }
        }
    }, [input]);

    return (
        <>


            <div className="buy-super-container">

                <div className="go-buy-super">
                    <h2>Split NFT</h2>
                </div>

                <div className="split-nft-input-container">

                    <div className="split-nft-input-first-line">


                        <p id="nft-prefix">Split NFT {nftLabel}</p>


                    </div>

                    <div className="split-nft-input-second-line">

                        <p id="split-input-prefix">into </p>
                        <input
                            type={"text"}
                            id={"split-input"}
                            value={input}
                            onChange={e => setInput(e.target.value)} // Added onChange handler
                            disabled={!enableInput}
                            placeholder="# of Splits"
                            style={{marginBottom: '0.625rem'}}
                        />
                        <p id='split-input-suffix'>
                            equivalent NFTs
                        </p>

                    </div>

                    <p> {error} </p>

                </div>

                <SplitNftButton
                    selectedAccount={selectedAccount}
                    enableButton = {enableButton}
                    YieldNftRaddy={YieldNftRaddy}
                    selectedNft={selectedNft}
                    numSplits={numSplits.toString()}
                />
            </div>
        </>
    );
};

SplitNftSectionV2.propTypes = {
    selectedAccount: PropTypes.string,
    selectedNft: PropTypes.object,
    YieldNftRaddy: PropTypes.string,
    enableInput: PropTypes.bool
};

const MemoizedSplitNftSection = React.memo(SplitNftSectionV2);

export default MemoizedSplitNftSection;