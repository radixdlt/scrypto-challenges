import { useState, useEffect } from "react";
import ClaimHello from "../components/ClaimHello.jsx";
import { useAccount } from "../AccountContext.jsx";

const HelloTokenSection = () => {
  const { accounts, selectedAccount, setSelectedAccount } = useAccount();
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const [enableButtons, setEnableButtons] = useState(false);
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
        <h2>Get Your Hello Token</h2>
        <p className="head-text">
          Claim your <span className="hello-token-pink">Hello Token</span>
        </p>
      </div>
      <div className="hello-token-container">
        <div className="hello-token-left-col">
          <h3>Have you Setup Dev Mode?</h3>
          <p>
            In order to receive your{" "}
            <span className="hello-token-pink-sm">Hello Token</span> please set
            up Dev Mode first using the steps above.
          </p>
          {/* <!-- ************ Custom Select ****************** --> */}

          <>
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
                <span className={selectedAccount ? "arrow-account" : "arrow"} />
              </button>
              {dropdownOpen && (
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
            </div>

            <ClaimHello
              selectedAccount={selectedAccount}
              enableButtons={enableButtons}
            />
          </>
        </div>
        {/* <!-- vert-bar --> */}
        <div
          style={{
            width: "0%",
            height: "60%",
            opacity: 0.3,
            borderLeft: "1px solid white",
          }}></div>
        {/* <!-- vert-bar --> */}
        <div className="hello-tokens">
          <img src="src/assets/hello-tokens.png" alt="hello tokens" />
        </div>
      </div>
    </>
  );
};

export default HelloTokenSection;
