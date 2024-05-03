import { NavLink } from 'react-router-dom';

const PrimaryNavbar = () => {
  return (
    <div id="navbar">

        <div id="navbar-container">
            <img src="http://www.assets.floww.fi/images/logo/tp/white/floww.svg" alt="floww logo"/>
        </div>

        <div id="navbar-link-container">
            <ul id="navbar-links">
                <li><NavLink to="/"> Home </NavLink></li>
                <li><NavLink to="/super/"> SUPER </NavLink></li>
                <li><NavLink to="/DevsOnly"> Dev </NavLink></li>
                {/* <li><NavLink to="/DApp"> DApp </NavLink></li> */}
                <li><NavLink to="/Docs"> Docs </NavLink></li>

            </ul>
        </div>

        <div id="connect-btn">
            <radix-connect-button/>
        </div>

    </div>
  );
};

export default PrimaryNavbar;
//       //"#/assets/images/logo/transparent/white/floww.svg" alt="dev mode setup"