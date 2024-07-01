import {useState, useEffect} from "react";
import BuySuper from "../components/BuySuper.jsx";
import ExchangeRatePic from "../components/ExchangeRatePic.jsx";
import AccountDropdown from "../components/AccountDropdown.jsx";
import {useAccount} from "../hooks/useAccount.jsx";

/**
 * BuySuperSection component that provides a section for users to purchase SUPER.
 *
 * @returns {JSX.Element} The rendered "Buy Super" section component.
 */
const BuySuperSection = () => {
  const { accounts } = useAccount();
  const [selectedAccount, setSelectedAccount] = useState(null); // State to manage the selected account
  const [enableButtons, setEnableButtons] = useState(false); // State to enable/disable buttons
  const [xrdAmount, setXrdAmount] = useState(''); // State to manage the XRD amount input
  const [error, setError] = useState(''); // State to manage input errors

  useEffect(() => {
    // Automatically enable buttons if accounts are available
    setEnableButtons(accounts.length > 0);
  }, [accounts]);

  useEffect(() => {
    // Enable or disable buttons based on account availability
    if (accounts.length > 0) {
      setEnableButtons(true);
    } else {
      setEnableButtons(false);
    }
  }, [accounts]);

// Helper function to check if a value is numeric
  const isNumeric = num => !isNaN(num);

  const handleChange = (e) => {
    const val = e.target.value.trim();
    if (val === '' || (isNumeric(val))) {
      setXrdAmount(val);
      setError('');
    } else {
      setError('Please enter a numeric value.');
    }
  };

  return (
      <>

        <div className="buy-super-container">

          <div className="go-buy-super">

            <h2>Go</h2> <h2 className='h2-cyan'>SUPER</h2>

          </div>

          <ExchangeRatePic/>

          <div className='buy-super-input-container'>

            <AccountDropdown
                selectedAccount={selectedAccount}
                setSelectedAccount={setSelectedAccount}
                enableDropdown={enableButtons}
            />

            <div className="buy-super-input-wrapper">

              <input
                  type={"text"}
                  id={"buy-super-input"}
                  onChange={handleChange}
                  value={xrdAmount}
                  placeholder="Enter XRD Amount"
                  style={{marginBottom: '0.625rem'}}
              />

              <p id={'input-suffix'}>
                XRD
              </p>

            </div>

            <BuySuper
                selectedAccount={selectedAccount}
                enableButtons={enableButtons}
                xrdAmount={xrdAmount}
                error={error}
            />

          </div>


        </div>

      </>
  );
};

export default BuySuperSection;
