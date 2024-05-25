import InstantiateSuper from "../components/instantiateSuper.jsx";
import PropTypes from "prop-types";

const InstantiateSection = ({ selectedAccount }) => {

    return (
        <>
            <div className="owner-page-container" >

                <div className="choose-owner-heading-section">

                    <h2>Instantiate SUPER</h2>

                    <p className="head-text">
                        Instantiate <span className="hello-token-pink">SUPER</span> component.
                    </p>

                    <div className="owner-page-button-container">
                            <InstantiateSuper selectedAccount={selectedAccount} />
                    </div>

                </div>

            </div>
        </>
    );
};

InstantiateSection.propTypes = {
    selectedAccount: PropTypes.string,  // It's common to mark this as isRequired if the component cannot function without it
    enableButtons: PropTypes.bool.isRequired  // Making sure to denote the importance of this prop
};

export default InstantiateSection;
