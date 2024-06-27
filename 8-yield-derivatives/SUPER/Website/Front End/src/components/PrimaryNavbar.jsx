import { useState, useEffect } from 'react';
import { NavLink } from 'react-router-dom';


/**
 * PrimaryNavbar component that provides the primary navigation bar for the DApp.
 * It includes links to different pages of the application and a connect button for
 * connecting the Radix Wallet.
 * The navbar layout adapts for mobile and desktop views.
 *
 * @returns {JSX.Element} The rendered PrimaryNavbar component.
 */
const PrimaryNavbar = () => {
    const [isMobile, setIsMobile] = useState(window.innerWidth < 850);

    useEffect(() => {
        // Handler to update the isMobile state based on window width
        const handleResize = () => {
            setIsMobile(window.innerWidth < 850);
        };

        // Attach the resize event listener
        window.addEventListener('resize', handleResize);

        // Cleanup the event listener on a component unmount
        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, []);

    if (isMobile) {
        // Mobile layout
        return (

            <div id="navbar">

                <div id="navbar-container">

                    <img src="https://assets.floww.fi/images/logo/svg/tp/FLOWW_BLUE.svg" alt="floww logo"/>

                    <div id="connect-btn">
                        <radix-connect-button/>
                    </div>

                </div>

                <div id="navbar-link-container">
                    <ul id="navbar-links">
                        <li><NavLink to="/"> Home </NavLink></li>
                        <li><NavLink to="/super/"> SUPER </NavLink></li>
                        <li><NavLink to="/DevsOnly"> Dev </NavLink></li>
                        <li><NavLink to="/Docs"> Docs </NavLink></li>
                    </ul>
                </div>


            </div>
        );
    } else {
        // Desktop layout
        return (

            <div id="navbar">


                <img src="https://assets.floww.fi/images/logo/svg/tp/FLOWW_WHITE.svg" alt="floww logo"/>

                <div id="navbar-link-container">
                    <ul id="navbar-links">
                        <li><NavLink to="/"> Home </NavLink></li>
                        <li><NavLink to="/super/"> SUPER </NavLink></li>
                        <li><NavLink to="/DevsOnly"> Dev </NavLink></li>
                        <li><NavLink to="/Docs"> Docs </NavLink></li>
                    </ul>
                </div>

                <div id="connect-btn">
                    <radix-connect-button/>
                </div>

            </div>
        );
    }
}

export default PrimaryNavbar;
