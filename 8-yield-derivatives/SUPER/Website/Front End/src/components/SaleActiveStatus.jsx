import {useSaleCompleted, useStarted} from "../hooks/useComponentDetails.js";

const SaleActiveStatus = () => {
    const saleCompleted = useSaleCompleted();
    const saleStarted = useStarted();

    const getStatusColor = () => {
        if (saleCompleted) {
            return "pink-text"; // Active
        } else if (saleStarted) {
            return "aqua-text"; // In Progress
        } else {
            return "cyan-text"; // Not Yet Started
        }
    };

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
