import { NavLink } from 'react-router-dom';

const SecondaryNavbar = () => {
    return (
        <div id="secondary-navbar">
            <div id="secondary-navbar-link-container">
                <ul id="secondary-navbar-links">
                    <li><NavLink to="/super/buy"> Buy </NavLink></li>
                    <li><NavLink to="/super/manage"> Manage </NavLink></li>
                    <li><NavLink to="/super/superv2"> Manage2 </NavLink></li>
                </ul>

            </div>
        </div>
    );
};

export default SecondaryNavbar;
//       //"#/assets/images/logo/transparent/white/floww.svg" alt="dev mode setup" 