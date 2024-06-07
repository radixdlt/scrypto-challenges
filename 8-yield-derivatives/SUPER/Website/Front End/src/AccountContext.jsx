import {createContext, useEffect, useMemo, useState} from 'react';
import {DataRequestBuilder} from "@radixdlt/radix-dapp-toolkit";
import {useRdt} from './hooks/useRdt.js';
import PropTypes from "prop-types";

// Create a context with a default value of null
export const AccountContext = createContext(null);

const AccountProvider = ({ children }) => {
  const [accounts, setAccounts] = useState([]);
  const [selectedAccount, setSelectedAccount] = useState(null);

  const rdt = useRdt();

  useEffect(() => {
    // Set the request data to get at least one account
    rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1));

    // Subscribe to wallet data updates
    const subscription = rdt.walletApi.walletData$.subscribe((walletData) => {
      console.log("subscription wallet data: ", walletData);

      // Update the accounts state with the received wallet data
      setAccounts(walletData && walletData.accounts ? walletData.accounts : []);
    });

    // Unsubscribe from the wallet data updates on cleanup
    return () => subscription.unsubscribe();
  }, [rdt]);

  // Memoizing the context value to optimize performance
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

// Define prop types
AccountProvider.propTypes = {
  children: PropTypes.node.isRequired
};

export { AccountProvider };
