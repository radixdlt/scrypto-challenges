import {createContext} from "react";


const MainNetMode = 0; //{  True = 1 = Mainnet  |  False = 0 = Stokenet  }

const xrdMainnet = "resource_xrd_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
const xrdStokenet = "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
export const XrdAddress = createContext(MainNetMode === 1 ? xrdMainnet : xrdStokenet);

export const gatewayApiContext = createContext(null);

export const RdtContext = createContext(null);

export const newSuperTxID = createContext(import.meta.env.VITE_PUBLISH_TX_ID);
export const packageAddy = createContext(import.meta.env.VITE_PKG_ADDY);
export const DappDefinitionCaddy = createContext(import.meta.env.VITE_DAPP_ID);

export const UpdateTriggerContext = createContext();
