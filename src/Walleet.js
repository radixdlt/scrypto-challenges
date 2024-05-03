import React from 'react';

const WalletButton = () => {
  const connectWallet = () => {
    // Your wallet connection logic goes here
    // For example, you can call a function to connect the wallet
    console.log('Connecting wallet...');
  };

  return (
    <button onClick={connectWallet}>Connect Wallet</button>
  );
};

export default WalletButton;
