import {useContext, useEffect, useMemo} from "react";
import {XrdAddress} from "../context/contexts.jsx";

//import { useSaleDetails } from "../context/SaleDetailProvider.jsx";
import {useSaleDetails} from "./useSaleDetails.js"
import {DappDefinitionCaddy} from "../context/fromEnv/DappDefinitionCaddy.jsx";
import {saleLength} from "../context/fromEnv/saleLength.jsx";
import {packageAddy} from "../context/fromEnv/PackageAddy.jsx";
import {newSuperTxID} from "../context/fromEnv/NewSuperTxID.jsx";


/*#region Constants from .env*/

export const useNewSuperTxID = () => {
    return useContext(newSuperTxID);
} 

export const useSaleLength = () => {
    return useContext(saleLength)
}

export const usePackageAddy = () => {

    return useContext(packageAddy);
};

export const useXrdAddy = () => {
    return useContext(XrdAddress);
}

/*#endregion ComponentAddresses*/

/*#region ComponentAddresses*/

export const useComponentAddy = () => {
    const saleDetails = useSaleDetails();
    
    useEffect(() => {
             if (saleDetails) {
            //console.log("component_caddy:", saleDetails.component_caddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.component_caddy : null;
};

export const useDappDefinitionCaddy = () => {
    return useContext(DappDefinitionCaddy);
};

export const usePoolCaddy = () => {
    const saleDetails = useSaleDetails();
    
    useEffect(() => {
                     if (saleDetails) {
                //console.log("pool_caddy:", saleDetails.pool_caddy);
            }
        }, [saleDetails]);

 return saleDetails ? saleDetails.pool_caddy : null;
};

/*#endregion */

/*#region Badge ResourceAddys*/


export const useOwnerBadgeRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
                     if (saleDetails) {
                //console.log("owner_badge_raddy:", saleDetails.owner_badge_raddy);
            }
        }, [saleDetails]);

 return saleDetails ? saleDetails.owner_badge_raddy : null;
};

export const useComponentBadgeRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
             if (saleDetails) {
            //console.log("component_badge_raddy:", saleDetails.component_badge_raddy);
        }
    }, [saleDetails]);

 return saleDetails ? saleDetails.component_badge_raddy : null;
};

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


export const useSuperRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("super_raddy:", saleDetails.super_raddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.super_raddy : null;
};

export const useSuperYRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("super_y_raddy:", saleDetails.super_y_raddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.super_y_raddy : null;
};

export const useSuperTRaddy = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("super_t_raddy:", saleDetails.super_t_raddy);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.super_t_raddy : null;
};

export const useYieldNftRaddy = () => {
    const saleDetails = useSaleDetails();
    return useMemo(() => saleDetails ? saleDetails.yield_nft_raddy : null, [saleDetails]);
};

/*#endregion */

/*#region Sale Status Booleans*/


export const useStarted = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_started:", saleDetails.sale_started);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_started : null;
};

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

export const useStartTimeUnix = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_start_time_unix:", saleDetails.sale_start_time_unix);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_start_time_unix : null;
};

export const useStartTimeUtc = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_start_time_utc:", saleDetails.sale_start_time_utc);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_start_time_utc : null;
};

export const useEndTimeUnix = () => {
    const saleDetails = useSaleDetails();
    useEffect(() => {
        if (saleDetails) {
            //console.log("sale_end_time_unix:", saleDetails.sale_end_time_unix);
        }
    }, [saleDetails]);

    return saleDetails ? saleDetails.sale_end_time_unix : null;
};

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
