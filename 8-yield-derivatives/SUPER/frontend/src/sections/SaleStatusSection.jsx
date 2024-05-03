import React, { useState } from "react";
import StartSale from "../components/StartSale.jsx";
import PropTypes from "prop-types";
import SaleActiveStatus from "../components/SaleActiveStatus.jsx";
import EndSale from "../components/EndSale.jsx";
import EndCountdown from "../components/EndCountdown.jsx";

const SaleStatusSection = ({ selectedAccount, setSelectedAccount, enableButtons }) => {
    const [error, setError] = useState('');

    return (
        <>
            <div className="owner-page-container">

                <div className="choose-owner-heading-section">
                    <h2>Sale Details</h2>
                    <div className="head-text">
                        <SaleActiveStatus />
                        <EndCountdown />
                    </div>
                </div>

                <div className="sale-status-button-container">
                    <StartSale selectedAccount={selectedAccount} enableButtons={enableButtons} />
                    <EndSale selectedAccount={selectedAccount} enableButtons={enableButtons} />
                    {error && <div style={{ color: 'red', marginTop: '1rem' }}>{error}</div>}
                </div>


            </div>
        </>
    );
};

SaleStatusSection.propTypes = {
    selectedAccount: PropTypes.string, // If `selectedAccount` is required for StartSale to function, consider using `.isRequired`
    setSelectedAccount: PropTypes.func,
    enableButtons: PropTypes.bool.isRequired
};

export default SaleStatusSection;
