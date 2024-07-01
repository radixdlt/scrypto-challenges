import {createContext, useContext, useEffect, useState} from "react";
import PropTypes from "prop-types";
import {UpdateTriggerContext} from "../updateTriggerContext.jsx";
import {getLatestSaleDetails} from "../../api/get.js";

// Create a context for sale details
export const SaleDetailsContext = createContext(null);


export const SaleDetailsProvider = ({ children }) => {
    const [saleDetails, setSaleDetails] = useState(null);
    const { trigger } = useContext(UpdateTriggerContext);

    useEffect(() => {
        // Function to fetch sale details
        const fetchSaleDetails = async () => {
            try {
                const response = await getLatestSaleDetails();
                setSaleDetails(response);
            } catch (error) {
                console.error('Error fetching sale details:', error);
            }
        };

        fetchSaleDetails();

        // Re-fetch sale details whenever the update trigger changes
    }, [trigger]);

    return (
        // Provide the sale details to children components
        <SaleDetailsContext.Provider value={saleDetails}>
            {children}
        </SaleDetailsContext.Provider>
    );
};

SaleDetailsProvider.propTypes = {
    children: PropTypes.node.isRequired,
};
