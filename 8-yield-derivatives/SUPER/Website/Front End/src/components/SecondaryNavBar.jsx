import { NavLink } from 'react-router-dom';

/**
 * SecondaryNavbar component that provides the secondary navigation bar for the DApp.
 * It includes links to the "Buy" and "Manage" pages within the "SUPER" section.
 *
 * @returns {JSX.Element} The rendered SecondaryNavbar component.
 */
const SecondaryNavbar = () => {
    return (
        <div id="secondary-navbar">
            <div id="secondary-navbar-link-container">
                <ul id="secondary-navbar-links">
                    <li><NavLink to="/super/buy"> Buy </NavLink></li>
                    <li><NavLink to="/super/manage"> Manage </NavLink></li>
                </ul>

            </div>
        </div>
    );
};

export default SecondaryNavbar;
