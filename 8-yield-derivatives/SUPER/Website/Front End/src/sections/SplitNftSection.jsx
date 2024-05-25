import React, {useEffect, useState} from "react";
import PropTypes from "prop-types";
import YieldNFTDropdown from "../components/YieldNFTDropdown.jsx";
import SplitNftButton from "../components/SplitNftButton.jsx";

const SplitNftSection = ({ selectedAccount, enableSelectNft, YieldNftRaddy }) => {

    const [numSplits, setNumSplits] = useState(0);
    const [error, setError] = useState('');
    const [selectedNft, setSelectedNft] = useState(null);
    const [enableInput, setEnableInput] = useState(false);
    const [enableButton, setEnableButton] = useState(false);
    const [input, setInput] = useState(""); // Changed to use state

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

                      <p id="nft-prefix">Split</p>
                      <YieldNFTDropdown
                          selectedAccount={selectedAccount}
                          enableSelectNft={enableSelectNft}
                          YieldNftRaddy={YieldNftRaddy}
                          setSelectedNft={setSelectedNft}
                          setEnableInput={setEnableInput}
                      />

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
                          NFTs
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

SplitNftSection.propTypes = {
    selectedAccount: PropTypes.string,
    enableSelectNft: PropTypes.bool,
    YieldNftRaddy: PropTypes.string,
};

const MemoizedSplitNftSection = React.memo(SplitNftSection);

export default MemoizedSplitNftSection;