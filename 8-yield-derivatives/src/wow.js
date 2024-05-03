import React, { useEffect, useState } from "react";
import { RadixDappToolkit, RadixIdentityManager } from '@radixdlt/application';

const Profile = () => {
  const [account, setAccount] = useState(null);

  useEffect(() => {
    // Instantiate the Radix Dapp Toolkit
    const rdt = RadixDappToolkit.initialize({
      nodeConnection: {
        websocketURL: 'ws://localhost:8080',
      },
    });

    // Load the user's account from Radix Wallet Connector
    async function loadAccount() {
      const identity = await RadixIdentityManager.loadFromWalletConnector();
      if (identity) {
        setAccount(identity.account);
      }
    }

    loadAccount();

    return () => {
      rdt.close();
    };
  }, []);

  const handleConnectWallet = async () => {
    try {
      // Connect to Radix Wallet Connector
      await RadixIdentityManager.connectToWalletConnector();
      // Reload the account after connecting
      loadAccount();
    } catch (error) {
      console.error("Error connecting to Radix Wallet Connector:", error);
    }
  };

  return (
    <div className="container">
      <div className="aboutUs">
        <div className="welcome">
          <h2>Welcome Back, Alex!</h2>
          <p>Wow! You spent 725 minutes with your favourite artist this year!</p>
        </div>
        <div className="grid-container">
          <div className="section" style={{ width: "100%" }}>
            <div className="profile">
              <img className="avatar2" src={avata} alt="Avatar" />
            </div>
          </div>
          <div className="section" style={{ width: "100%" }}>
            <div className="calculator">
              <img className="avatar3" src={graph} alt="Avatar" />
            </div>
            <div className="landing">
            </div>
          </div>
        </div>
        <div className="button-container">
          {/* Display connect wallet button if account is not loaded */}
          {!account && <button className="wallet-button" onClick={handleConnectWallet}>Connect Wallet</button>}
          {/* Display account information if loaded */}
          {account && (
            <div>
              <p>Account Name: {account.name}</p>
              <p>Account Address: {account.address}</p>
            </div>
          )}
        </div>
        {/* Add iframe to display external page */}
        <iframe
          title="Portfolio Accounts"
          src="https://launch.shardspace.app/portfolio/accounts"
          width="100%"
          height="600px" // Adjust height as needed
          style={{ border: "none", marginTop: "20px" }}
        />
      </div>
    </div>
  );
};

export default Profile;
