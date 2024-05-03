import React, { useState, useEffect } from 'react';
import {
  DataRequestBuilder,
  RadixDappToolkit,
  RadixNetwork,
} from '@radixdlt/radix-dapp-toolkit';
import './profile.css';
import profilePicture from './profile.jpg'; // Import your profile picture




const Profile = () => {
  const [accountAddress, setAccountAddress] = useState('');
  const [walletConnected, setWalletConnected] = useState(false);
  const [error, setError] = useState('');
  const [insuranceContract, setInsuranceContract] = useState(null);
  const [rdt, setRdt] = useState(null);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [additionalData, setAdditionalData] = useState('');
  const [claimStatus, setClaimStatus] = useState('');
  const [stalkingInput, setStalkingInput] = useState('');
  const [stalkingResult, setStalkingResult] = useState('');
  const [premiumAmount, setPremiumAmount] = useState(0);
  const [payoutAmount, setPayoutAmount] = useState(0);
  const [riskLevel, setRiskLevel] = useState(0);
  const [showShardSpace, setShowShardSpace] = useState(false);

  const toggleShardSpace = () => {
    setShowShardSpace(!showShardSpace);
  };

  useEffect(() => {
    const dAppDefinitionAddress = 'account_tdx_2_12xxk4dqhg9dz53p745qhpr5tr2k2al4mpx3296tr8k78kna6rkcgsz';
    
    // Create a dapp configuration object for the Radix Dapp Toolkit
    const dappConfig = {
      networkId: RadixNetwork.Stokenet,
      applicationVersion: '1.0.0',
      applicationName: 'Infix',
      applicationDappDefinitionAddress: dAppDefinitionAddress,
      dAppDefinitionAddress,
    };

    // Instantiate DappToolkit to connect to the Radix wallet and network
    const rdtInstance = RadixDappToolkit(dappConfig);
    setRdt(rdtInstance);

    // Connect a user account when wallet is connected
    rdtInstance.walletApi.setRequestData(DataRequestBuilder.accounts().exactly(1));

    // Subscribe to updates to the user's shared wallet data
    const subscription = rdtInstance.walletApi.walletData$.subscribe({
      next: (walletData) => {
        if (walletData && walletData.accounts && walletData.accounts.length > 0) {
          setAccountAddress(walletData.accounts[0].address);
          setWalletConnected(true);
          setError(''); // Clear any previous errors
          console.log('Wallet connected successfully:', walletData.accounts[0].address);
        } else {
          setError('No account data found.');
          console.error('No account data found.');
        }
      },
      error: (err) => {
        setError('Error connecting to wallet: ' + err.message);
        console.error('Error connecting to wallet:', err.message);
      }
    });
    // Cleanup subscription on unmount
    return () => {
      subscription.unsubscribe();
    };
  }, []);

  // Function to connect to the wallet
  const connectToWallet = async () => {
    try {
      // Request the wallet to connect
      await rdt.walletApi.connectWallet();
    } catch (error) {
      setError('Error connecting to wallet: ' + error.message);
    }
  };

  // Function to create the insurance contract
  const createInsuranceContract = async (insuredDomain, premiumAmount, payoutAmount, riskLevel) => {
    const dAppDefinitionAddress = 'account_tdx_2_12xxk4dqhg9dz53p745qhpr5tr2k2al4mpx3296tr8k78kna6rkcgsz';

    const result = await rdt.walletApi.sendTransaction({
        transactionManifest: `
            CALL_FUNCTION
                Address("${dAppDefinitionAddress}")
                "parametric_insurance::ParametricInsurance"
                "new"
                String("${insuredDomain}")
                Decimal("${premiumAmount}")
                Decimal("${payoutAmount}")
                Decimal("${riskLevel}");
        `,
        version: 1,
    });

    if (result.isErr()) {
        console.error('Error creating insurance contract:', result.error);
        return null;
    }

    setInsuranceContract(result.value.global);
    console.log('Insurance contract created:', result.value.global);
    return result.value.global;
  };

  // Function to make a claim on the insurance contract
  const makeClaim = async () => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'make_claim',
        args: [],
    });

    if (result.isErr()) {
        console.error('Error making claim:', result.error);
        return;
    }

    console.log('Claim made successfully:', result.value);
  };

  // Function to check if the insurance contract has been claimed
  const isClaimed = async () => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'is_claimed',
        args: [],
    });

    if (result.isErr()) {
        console.error('Error checking claim status:', result.error);
        return;
    }

    console.log('Is claimed:', result.value);
  };

  // Function to cancel the insurance contract
  const cancelContract = async () => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'cancel_contract',
        args: [],
    });

    if (result.isErr()) {
        console.error('Error canceling contract:', result.error);
        return;
    }

    console.log('Contract canceled successfully:', result.value);
  };

  // Function to trigger insurance payout based on predefined conditions
  const triggerPayout = async (conditionMet) => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'trigger_payout',
        args: [conditionMet],
    });

    if (result.isErr()) {
        console.error('Error triggering payout:', result.error);
        return;
    }

    console.log('Payout triggered successfully:', result.value);
  };

  // Function to monitor liquidity and trigger insurance payout if a significant drop is detected
  const monitorLiquidity = async (liquidityDropThreshold, timeFrameHours, currentLiquidity) => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'monitor_liquidity',
        args: [liquidityDropThreshold, timeFrameHours, currentLiquidity],
    });

    if (result.isErr()) {
        console.error('Error monitoring liquidity:', result.error);
        return;
    }

    console.log('Liquidity monitored successfully:', result.value);
  };

  // Function to monitor market volatility and trigger insurance payout if volatility exceeds threshold
  const monitorMarketVolatility = async (volatilityThreshold, currentVolatility) => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'monitor_market_volatility',
        args: [volatilityThreshold, currentVolatility],
    });

    if (result.isErr()) {
        console.error('Error monitoring market volatility:', result.error);
        return;
    }

    console.log('Market volatility monitored successfully:', result.value);
  };

  // Function to update the premium amount of the insurance contract
  const updatePremiumAmount = async (newPremiumAmount) => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'update_premium_amount',
        args: [newPremiumAmount],
    });

    if (result.isErr()) {
        console.error('Error updating premium amount:', result.error);
        return;
    }

    console.log('Premium amount updated successfully:', result.value);
  };

  // Function to update the payout amount of the insurance contract
  const updatePayoutAmount = async (newPayoutAmount) => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'update_payout_amount',
        args: [newPayoutAmount],
    });

    if (result.isErr()) {
        console.error('Error updating payout amount:', result.error);
        return;
    }

    console.log('Payout amount updated successfully:', result.value);
  };

  // Function to automate claim processing if conditions are met
  const automateClaimProcessing = async (autoClaimCondition) => {
    if (!insuranceContract) {
        console.error('Insurance contract not created yet');
        return;
    }

    const result = await insuranceContract.call({
        method: 'automate_claim_processing',
        args: [autoClaimCondition],
    });

    if (result.isErr()) {
        console.error('Error automating claim processing:', result.error);
        return;
    }

    console.log('Claim processing automated successfully:', result.value);
  };

  // Function to handle sign up
  const signUp = async () => {
    try {
      // Code to create a new insurance contract
      const result = await createInsuranceContract(username, premiumAmount, payoutAmount, riskLevel);
      // Handle success
      console.log('Insurance contract created:', result);
    } catch (error) {
      // Handle error
      console.error('Error creating insurance contract:', error);
    }
  };

  // Function to handle claiming money
  const claimMoney = async () => {
    try {
      // Code to trigger the claim process
      // For example:
      const result = await insuranceContract.call({
        method: 'make_claim',
        args: [],
      });
      // Handle success
      console.log('Claim made successfully:', result.value);
      setClaimStatus('Claim made successfully');
    } catch (error) {
      // Handle error
      console.error('Error making claim:', error);
      setClaimStatus('Error making claim');
    }
  };

  // Function to handle stalking
  const stalk = async (id) => {
    try {
      // Code to fetch and display information about the specified ID
      // For example:
      const info = await fetchInformation(id);
      // Handle success
      console.log('Stalking result:', info);
      setStalkingResult(info);
    } catch (error) {
      // Handle error
      console.error('Error stalking:', error);
      setStalkingResult('Error stalking');
    }
  };

  // Function to handle change in stalking input
  const handleStalkInputChange = (event) => {
    // Update the state with the input value
    // For example:
    setStalkingInput(event.target.value);
  };
// Define fetchInformation function
const fetchInformation = async (id) => {
  // Implementation of fetchInformation function
};

// Define handleContractInteractions function
const handleContractInteractions = () => {
  // Implementation of handleContractInteractions function
};

return (
  <div className="container">
    <div className="profile-details" style={{ marginBottom: '20px', display: 'flex', alignItems: 'center' }}>
      <div className="profile">
        <img src={profilePicture} alt="Profile" className="avatar" style={{ width: '200px', height: '200px', borderRadius: '80%', marginBottom: '10px' }} />
      </div>

      <div className="button-row" style={{ display: 'flex', justifyContent: 'space-between', width: '100%' }}>
        <div className="form" style={{ width: '25%', padding: '15px' }}>
          <h2>Sign Up For Insurance</h2>
          <input type="text" placeholder="Your xrd Domain" value={username} onChange={(e) => setUsername(e.target.value)} />
          <input type="number" placeholder="Premium Amount" value={premiumAmount} onChange={(e) => setPremiumAmount(e.target.value)} />
          <input type="number" placeholder="Payout Amount" value={payoutAmount} onChange={(e) => setPayoutAmount(e.target.value)} />
          <input type="number" placeholder="Risk Level" value={riskLevel} onChange={(e) => setRiskLevel(e.target.value)} />
          <button className="wallet-button" onClick={signUp} style={{ padding: '15px' }}>
            Create Insurance
          </button>
        </div>

        <div className="wallet-section" style={{ width: '25%' }}>
          {error && walletConnected ? (
            <div>
              <p>Wallet Connected Successfully</p>
              <p>Account Address: {accountAddress}</p>
              <button className="wallet-button" onClick={handleContractInteractions} style={{ width: '100%', padding: '15px' }}>
                Interact with Contract
              </button>
            </div>
          ) : (
            <div>
              <button className="wallet-button" onClick={connectToWallet} style={{ width: '100%', padding: '15px' }}>
                Connect To Wallet
              </button>
            </div>
          )}
        </div>

        <div style={{ width: '25%' }}>
          <button className="wallet-button" onClick={claimMoney} style={{ padding: '15px' }}>
            Claim Your Crypto
          </button>
        </div>
      </div>

      <div className="stalk-section" style={{ width: '25%', padding: '15px' }}>
      <h2>Stalk Your Crypto</h2>

        <input type="text" placeholder="Enter Amount to stalk" onChange={handleStalkInputChange} />
        <button className="wallet-button" onClick={() => stalk(stalkingInput)} style={{ padding: '15px' }}>
          Stalk Your Crypto
        </button>
        {stalkingResult && <p>{stalkingResult}</p>}
      </div>
    </div>

    <button className="wallet-button" onClick={toggleShardSpace} style={{ width: '100%', marginTop: '20px', padding: '15px' }}>
      Experience ShardSpace
    </button>

    {showShardSpace && (
      <iframe
        title="Portfolio Accounts"
        src="https://launch.shardspace.app/portfolio/accounts"
        width="100%"
        height="600px"
        style={{ border: 'none', marginTop: '20px' }}
      />
    )}
  </div>
);


};

export default Profile;
