import {createContext, useContext, useEffect, useState} from "react";
import PropTypes from "prop-types";
import {UpdateTriggerContext} from "../contexts.jsx";
import {getLatestSaleDetails} from "../../api/get.js";


export const SaleDetailsContext = createContext(null);


export const SaleDetailsProvider = ({ children }) => {
    const [saleDetails, setSaleDetails] = useState(null);
    const { trigger } = useContext(UpdateTriggerContext);

    useEffect(() => {
        console.log("SaleDetailsProvider got shot, updating state");
        const fetchSaleDetails = async () => {
            try {
                const response = await getLatestSaleDetails();
                setSaleDetails(response);
            } catch (error) {
                console.error('Error fetching sale details:', error);
            }
        };

        fetchSaleDetails();
    }, [trigger]);

    return (
        <SaleDetailsContext.Provider value={saleDetails}>
            {children}
        </SaleDetailsContext.Provider>
    );
};
SaleDetailsProvider.propTypes = {
    children: PropTypes.node.isRequired,
};
