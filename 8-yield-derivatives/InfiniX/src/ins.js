import React, { useState, useEffect } from 'react';
import { RadixDappToolkit, RadixNetwork } from "@radixdlt/radix-dapp-toolkit";
import './TradingBotComponent.css'; // Import CSS for styling
import logo from './logo.png'; // Import your logo image

const TradingBotComponent = () => {
  const [response, setResponse] = useState('');
  const [loading, setLoading] = useState(false);
  const [rdt, setRdt] = useState(null);

  const [contractDetails, setContractDetails] = useState(null);
  const [claimingInsurance, setClaimingInsurance] = useState(false);
  const [signingContract, setSigningContract] = useState(false);

  useEffect(() => {
    const initializeRadixDappToolkit = async () => {
      try {
        const rdtInstance = RadixDappToolkit({
          dAppDefinitionAddress: "account_tdx_2_12xxk4dqhg9dz53p745qhpr5tr2k2al4mpx3296tr8k78kna6rkcgsz",
          networkId: RadixNetwork.Stokenet,
          applicationName: "parametric_insurance",
          applicationVersion: "1.0.0",
        });
        setRdt(rdtInstance);
      } catch (error) {
        console.error('Error initializing RadixDappToolkit:', error);
      }
    };

    initializeRadixDappToolkit();
  }, []);

  const handleRunBot = async () => {
    if (!rdt) return;

    setLoading(true);
    try {
      // Simulate bot response (replace with actual bot interaction)
      setTimeout(() => {
        setResponse('Bot: Buy signal detected for BTC/USD');
        setLoading(false);
      }, 2000);
    } catch (error) {
      console.error('Error running bot:', error);
      setLoading(false);
    }
  };

  const handleCreateContract = async () => {
    setSigningContract(true);
    // Implement logic to create a new contract
    // Once the contract is created, setSigningContract(false);
  };

  const handleViewDetails = async () => {
    // Implement logic to fetch and display contract details
    // Set contractDetails with the fetched details
  };

  const handleClaimInsurance = async () => {
    setClaimingInsurance(true);
    // Implement logic to claim insurance for a contract
    // Once the insurance is claimed, setClaimingInsurance(false);
  };

  return (
    <div className="trading-bot-container">
      <div className="trading-bot-form">
      <img src={logo} alt="InfiniX Logo" className="logo" style={{ width: '150px', height: '150px' }} />

        <h2>Welcome to InfiniX</h2>
        <p style={{ fontSize: '20px' }}>Instant Insurance, Infinite Possibilities!</p>

       
        {response && <div className="bot-response">{response}</div>}
        <h2>Your Insurance Platform</h2>
        <button onClick={handleCreateContract} disabled={signingContract}>
          {signingContract ? 'Signing Up Contract...' : 'Sign Up Contract'}
        </button>
        <button onClick={handleViewDetails}>View Contract Details</button>
        <button onClick={handleClaimInsurance} disabled={claimingInsurance}>
          {claimingInsurance ? 'Claiming Insurance...' : 'Claim Insurance'}
        </button>
        {contractDetails && (
          <div className="contract-details">
            {/* Display contract details here */}
          </div>
        )}
      </div>
    </div>
  );
};

export default TradingBotComponent;

