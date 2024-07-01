import {useContext} from "react";
import {AccountContext} from "../AccountContext.jsx";


/**
 * Custom hook to access the AccountContext.
 *
 * @returns {Object} The value of the AccountContext.
 */
export const useAccount = () => useContext(AccountContext);