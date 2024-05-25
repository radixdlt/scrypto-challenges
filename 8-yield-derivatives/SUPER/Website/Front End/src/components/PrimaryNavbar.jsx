import { useState, useEffect } from 'react';
import { NavLink } from 'react-router-dom';



const PrimaryNavbar = () => {
    const [isMobile, setIsMobile] = useState(window.innerWidth < 850);

    useEffect(() => {
        const handleResize = () => {
            setIsMobile(window.innerWidth < 850);
        };
        window.addEventListener('resize', handleResize);
        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, []);

    if (isMobile) {
        // Mobile layout
        return (

            <div id="navbar">

                <div id="navbar-container">

                    <img src="http://www.assets.floww.fi/images/logo/svg/tp/white/floww.svg" alt="floww logo"/>

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


                <img src="http://www.assets.floww.fi/images/logo/svg/tp/white/floww.svg" alt="floww logo"/>

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
