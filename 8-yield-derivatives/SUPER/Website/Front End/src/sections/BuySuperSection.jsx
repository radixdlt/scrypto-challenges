import {useState, useEffect} from "react";
import BuySuper from "../components/BuySuper.jsx";
import ExchangeRatePic from "../components/ExchangeRatePic.jsx";
import AccountDropdown from "../components/AccountDropdown.jsx";
import {useAccount} from "../hooks/useAccount.jsx";


const BuySuperSection = () => {
  const { accounts } = useAccount();
  const [selectedAccount, setSelectedAccount] = useState(null);
  const [enableButtons, setEnableButtons] = useState(false);
  const [xrdAmount, setXrdAmount] = useState('');
  const [error, setError] = useState('');

  useEffect(() => {
    // Automatically enable buttons if accounts are available
    setEnableButtons(accounts.length > 0);
  }, [accounts]);




  useEffect(() => {
    if (accounts.length > 0) {
      setEnableButtons(true);
    } else {
      setEnableButtons(false);
    }
  }, [accounts]); // Only re-run the effect if count changes


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
