
// Create a context for the sale details
import {createContext, useContext, useEffect, useState} from "react";
import {getLatestSaleDetails} from "../api/posts.js";
import PropTypes from "prop-types";
import {UpdateTriggerContext} from "./contexts.jsx";

export const SaleDetailsContext = createContext();

export const SaleDetailsProvider = ({ children }) => {
    const [saleDetails, setSaleDetails] = useState(null);
    const {trigger} = useContext(UpdateTriggerContext); //new

    useEffect(() => {
        // Fetch sale details from MongoDB
        const fetchSaleDetails = async () => {
            try {
                const response = await getLatestSaleDetails();
                setSaleDetails(response); // Update sale details context with fetched data
                console.log("from SALEDETAILSPROVIDER",response);
            } catch (error) {
                console.error('Error fetching sale details:', error);
            }
        };

        fetchSaleDetails();
    }, [trigger]); // Run effect only once on component mount


    SaleDetailsProvider.propTypes = {
        children: PropTypes.node.isRequired, // Add children prop validation
    };

    // Return the sale details context provider with its value set to saleDetails
    return (
        <SaleDetailsContext.Provider value={saleDetails}>
            {children}
        </SaleDetailsContext.Provider>
    );
};
