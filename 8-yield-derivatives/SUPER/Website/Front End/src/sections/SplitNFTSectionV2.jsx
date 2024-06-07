import React, {useEffect, useState} from "react";
import PropTypes from "prop-types";
import SplitNftButton from "../components/SplitNftButton.jsx";

const SplitNftSectionV2 = ({ selectedAccount, selectedNft, YieldNftRaddy, enableInput }) => {

    const [numSplits, setNumSplits] = useState(0);
    const [error, setError] = useState('');
    const [enableButton, setEnableButton] = useState(false);
    const [input, setInput] = useState(""); // Changed to use state
    const [nftLabel, setNftLabel] = useState("")

    useEffect(() => {
        if (selectedNft) {
            setNftLabel(selectedNft.label)
        }
        else {
            setNftLabel("")
        }
    }, [selectedNft]);

    const isNumeric = num => !isNaN(num);
    const isInteger = num => Number.isInteger(num);
    const isLowerThan50 = num => num <= 50;

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