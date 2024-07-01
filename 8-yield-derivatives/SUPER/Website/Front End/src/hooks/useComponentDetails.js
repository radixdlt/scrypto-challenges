import {useContext, useEffect, useMemo} from "react";

import {useSaleDetails} from "./useSaleDetails.js"
import {DappDefinitionCaddy} from "../context/fromEnv/DappDefinitionCaddy.jsx";
import {SaleLength} from "../context/fromEnv/SaleLength.jsx";
import {packageAddy} from "../context/fromEnv/PackageAddy.jsx";
import {newSuperTxID} from "../context/fromEnv/NewSuperTxID.jsx";
import {XrdAddressContext} from "../context/xrdAddressContext.jsx";


/*#region Constants from .env */

/**
 * Custom hook to access the newSuperTxID,
 * the transaction address where a _specific version_ of the SUPER Dapp is instantiated.
 * If you do not have one, don't worry, since you can instantiate a new one in the "Dev" page, and that TxId will be
 * communicated with the backend.
 *
 * The TxId can be inputted in the `.env` file.
 *
 * @returns {Object} The value of the newSuperTxID context.
 */
export const useNewSuperTxID = () => {
    return useContext(newSuperTxID);
}

/**
 * Custom hook to access the SaleLength context.
 * @deprecated **WARNING: DEPRECATED**
 * @returns {Object} The value of the SaleLength context.
 */
export const useSaleLength = () => {
    return useContext(SaleLength)
}

/**
 * Custom hook to access the published package Address from `.env` file.
 *
 * @returns {Object} The value of the packageAddy context.
 */
export const usePackageAddy = () => {
    return useContext(packageAddy);
};

/**
 * Custom hook to access the Xrd Address from `.env` file.
 *
 * @returns {Object} The value of the XrdAddressContext.
 */
export const useXrdAddy = () => {
    return useContext(XrdAddressContext);
}

/*#endregion ComponentAddresses*/


/*#region Component Addresses */

/**
 * Custom hook to access the component address from sale details.
 *
 * @returns {string|null} The component address or null if not available.
 */
export const useComponentAddy = () => {
    const saleDetails = useSaleDetails();
    
    useEffect(() => {
             if (saleDetails) {
            //console.log("component_caddy:", saleDetails.component_caddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.component_caddy : null;
};

/**
 * Custom hook to access the Dapp Definition Component Addresses.
 *
 * @returns {Object} The value of the DappDefinitionCaddy context.
 */
export const useDappDefinitionCaddy = () => {
    return useContext(DappDefinitionCaddy);
};

/**
 * Custom hook to access the SUPERt (`OneResourcePool`) Component Addresses from sale details.
 *
 * @returns {string|null} The pool address or null if not available.
 */
export const usePoolCaddy = () => {
    const saleDetails = useSaleDetails();
    
    useEffect(() => {
                     if (saleDetails) {
                //console.log("pool_caddy:", saleDetails.pool_caddy);
            }
        }, [saleDetails]);

 return saleDetails ? saleDetails.pool_caddy : null;
};

/*#endregion Component Addresses */


/*#region Badge Resource Addys*/

/**
 * Custom hook to access the owner resource badge address from sale details.
 *
 * @returns {string|null} The owner badge address or null if not available.
 */
export const useOwnerBadgeRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
                     if (saleDetails) {
                //console.log("owner_badge_raddy:", saleDetails.owner_badge_raddy);
            }
        }, [saleDetails]);

 return saleDetails ? saleDetails.owner_badge_raddy : null;
};

/**
 * Custom hook to access the component badge resource address from sale details.
 *
 * @returns {string|null} The component badge address or null if not available.
 */
export const useComponentBadgeRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
             if (saleDetails) {
            //console.log("component_badge_raddy:", saleDetails.component_badge_raddy);
        }
    }, [saleDetails]);

 return saleDetails ? saleDetails.component_badge_raddy : null;
};

/**
 * Custom hook to access the database updater resource address from sale details.
 * Maybe useless to have in the frontend, but I kept it just in case I would need it.
 * @returns {string|null} The database updater address or null if not available.
 */
export const useDbUpdaterRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
         if (saleDetails) {
            //console.log("db_updater_raddy:", saleDetails.db_updater_raddy);
        }
    }, [saleDetails]);

 return saleDetails ? saleDetails.db_updater_raddy : null;
};

/*#endregion */


/*#region Tokens Resource Addys*/

/**
 * Custom hook to access the SUPER token address from sale details.
 *
 * @returns {string|null} The super token address or null if not available.
 */
export const useSuperRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("super_raddy:", saleDetails.super_raddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.super_raddy : null;
};

/**
 * Custom hook to access the SUPERy token resource address from sale details.
 *
 * @returns {string|null} The super Y token address or null if not available.
 */
export const useSuperYRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("super_y_raddy:", saleDetails.super_y_raddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.super_y_raddy : null;
};

/**
 * Custom hook to access the SUPERt token resource address from sale details.
 *
 * @returns {string|null} The SUPERt token resource address or null if not available.
 */
export const useSuperTRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("super_t_raddy:", saleDetails.super_t_raddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.super_t_raddy : null;
};

/**
 * Custom hook to access the yield NFT resource address from sale details.
 *
 * @returns {string|null} The yield NFT address or null if not available.
 */
export const useYieldNftRaddy = () => {
    const saleDetails = useSaleDetails();
    return useMemo(() => saleDetails ? saleDetails.yield_nft_raddy : null, [saleDetails]);
};

/*#endregion Constants from SaleDetails*/


/*#region Sale Status Booleans*/

/**
 * Custom hook to access the sale started status from sale details.
 *
 * @returns {boolean|null} The sale started status or null if not available.
 */
export const useSaleStarted = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_started:", saleDetails.sale_started);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_started : null;
};

/**
 * Custom hook to access the sale completed status from sale details.
 *
 * @returns {boolean|null} The sale completed status or null if not available.
 */
export const useSaleCompleted = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_completed:", saleDetails.sale_completed);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_completed : null;
};

/*#endregion Sale Status Booleans*/


/*#region Sale Status Times*/

/**
 * Custom hook to access the sale start time in Unix format from sale details.
 *
 * @returns {number|null} The sale start time in Unix format or null if not available.
 */
export const useStartTimeUnix = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_start_time_unix:", saleDetails.sale_start_time_unix);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_start_time_unix : null;
};

/**
 * Custom hook to access the sale start time in UTC format from sale details.
 *
 * @returns {string|null} The sale start time in UTC format or null if not available.
 */
export const useStartTimeUtc = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_start_time_utc:", saleDetails.sale_start_time_utc);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_start_time_utc : null;
};

/**
 * Custom hook to access the sale end time in Unix format from sale details.
 *
 * @returns {number|null} The sale end time in Unix format or null if not available.
 */
export const useEndTimeUnix = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_end_time_unix:", saleDetails.sale_end_time_unix);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_end_time_unix : null;
};

/**
 * Custom hook to access the sale end time in UTC format from sale details.
 *
 * @returns {string|null} The sale end time in UTC format or null if not available.
 */
export const useEndTimeUtc = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_end_time_utc:", saleDetails.sale_end_time_utc);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_end_time_utc : null;
};

/*#endregion Sale Status Times*/
