import React, { useState, useEffect } from 'react';
import { DataRequestBuilder, RadixDappToolkit, RadixNetwork } from "@radixdlt/radix-dapp-toolkit";

const TradingBotComponent = () => {
  const [messages, setMessages] = useState([]);
  const [inputText, setInputText] = useState('');
  const [rdt, setRdt] = useState(null);

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
        rdtInstance.walletApi.setRequestData(DataRequestBuilder.accounts().exactly(1));
      } catch (error) {
        console.error('Error initializing RadixDappToolkit:', error);
      }
    };

    initializeRadixDappToolkit();
  }, []);

  const handleSendMessage = async () => {
    try {
      if (!rdt || !rdt.walletApi || !rdt.walletApi.getWalletData().accounts.length) {
        console.error('RadixDappToolkit or walletApi not available.');
        return;
      }

      const accountAddress = rdt.walletApi.getWalletData().accounts[0].address;

      const manifest = `
        CALL_METHOD
            Address("$")
            "new"
        ;
        CALL_METHOD
            Address("${accountAddress}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP")
        ;
      `;

      const result = await rdt.walletApi.sendTransaction({
        transactionManifest: manifest,
        version: 1,
      });

      if (result.isErr()) throw result.error;
      console.log("Transaction result: ", result.value);
    } catch (error) {
      console.error('Error sending message:', error);
    }
  };

  return (
    <div>
      <div>
        {messages.map((message, index) => (
          <div key={index}>{message.text}</div>
        ))}
      </div>
      <input value={inputText} onChange={(e) => setInputText(e.target.value)} />
      <button onClick={handleSendMessage}>Send</button>
    </div>
  );
};

export default TradingBotComponent;
