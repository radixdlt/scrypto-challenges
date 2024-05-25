import {createContext, useEffect, useMemo, useState} from 'react';
import {DataRequestBuilder} from "@radixdlt/radix-dapp-toolkit";
import {useRdt} from './hooks/useRdt';

export const AccountContext = createContext(null);

// eslint-disable-next-line react/prop-types
const AccountProvider = ({ children }) => {
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



  // Memoizing the context value
  const contextValue = useMemo(() => ({
    accounts,
    setAccounts,
    selectedAccount,
    setSelectedAccount
  }), [accounts, selectedAccount, setAccounts, setSelectedAccount]);

  return (
      <AccountContext.Provider value={contextValue}>
        {children}
      </AccountContext.Provider>
  );
};

export { AccountProvider };
