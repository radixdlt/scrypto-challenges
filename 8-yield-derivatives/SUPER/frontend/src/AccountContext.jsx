import React, { createContext, useContext, useState, useEffect } from 'react';
import { DataRequestBuilder } from "@radixdlt/radix-dapp-toolkit";
import { useRdt } from './hooks/useRdt';

const AccountContext = createContext();

export const useAccount = () => useContext(AccountContext);

export const AccountProvider = ({ children }) => {
  const [accounts, setAccounts] = useState([]);
  const [selectedAccount, setSelectedAccount] = useState(null);

  const rdt = useRdt();

  useEffect(() => {
    rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1));

    const subscription = rdt.walletApi.walletData$.subscribe((walletData) => {
      console.log("subscription wallet data: ", walletData);
      setAccounts(walletData && walletData.accounts ? walletData.accounts : []);
    });

    return () => subscription.unsubscribe();
  }, [rdt]);

  return (
    <AccountContext.Provider value={{ accounts, setAccounts, selectedAccount, setSelectedAccount }}>
      {children}
    </AccountContext.Provider>
  );
};
