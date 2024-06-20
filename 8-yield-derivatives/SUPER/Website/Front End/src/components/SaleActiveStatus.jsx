import {useSaleCompleted, useSaleStarted} from "../hooks/useComponentDetails.js";

/**
 * SaleActiveStatus component provides a visual indicator of the current sale status.
 * It displays whether the sale is completed, active, or not yet started by utilizing
 * two custom hooks: `useSaleCompleted` and `useSaleStarted`.
 *
 * @returns {JSX.Element} The rendered SaleActiveStatus component.
 */
const SaleActiveStatus = () => {
    const saleCompleted = useSaleCompleted(); // Hook to check if the sale is completed
    const saleStarted = useSaleStarted(); // Hook to check if the sale has started

    /**
     * Determines the CSS class for the status text based on the sale state.
     * @returns {string} The CSS class for the status text.
     */
    const getStatusColor = () => {
        if (saleCompleted) {
            return "pink-text"; // Sale completed
        } else if (saleStarted) {
            return "aqua-text"; // Sale in progress
        } else {
            return "cyan-text"; // Sale not yet started
        }
    };

    /**
     * Determines the status text based on the sale state.
     * @returns {string} The status text.
     */
    const getStatusText = () => {
        if (saleCompleted) {
            return "DONE";
        } else if (saleStarted) {
            return "ACTIVE";
        } else {
            return "THOON";
        }
    };

    return (
        <span className="sale-active-status" style={{display:"inline-flex"}}>
            <p>Sale Active Status:</p>
            <p className={`${getStatusColor()}`}>{getStatusText()}</p>
        </span>
    );
};

export default SaleActiveStatus;
