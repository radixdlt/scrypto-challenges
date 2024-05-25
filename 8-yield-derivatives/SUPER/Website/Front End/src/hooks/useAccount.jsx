import {useContext} from "react";
import {AccountContext} from "../AccountContext.jsx";

export const useAccount = () => useContext(AccountContext);