import React, { useState } from 'react';
import logo from './logo.png';
import { AiOutlineMenu, AiOutlineMinus } from 'react-icons/ai';
import { Link } from 'react-router-dom';

const Header = () => {
  const [click, setClick] = useState(false);
  const handleClick = () => setClick(!click);

  const handleNavigate = (path) => {
    // Logic for navigation if needed
  };

  return (
    <header className="header1">
      <div>
        <img
          src={logo}
          alt="Logo"
          className="logo"
          style={{
            width: '190px',
            height: '140px',
            borderRadius: '50%',
            marginTop: '15px',
            position: 'relative',
          }}
        />
      </div>

      <ul className={click ? 'nav-menu1 active1' : 'nav-menu1'}>
        <li>
          <button className="wallet-button" onClick={() => handleNavigate('/')}>
            Home
          </button>
        </li>
        <li>
          <button className="wallet-button" onClick={() => handleNavigate('/our-services')}>
            Our Services
          </button>
        </li>
        <li>
          <button className="wallet-button" onClick={() => handleNavigate('/profile')}>
            InfiniX NFTs
          </button>
        </li>
        <li>
        </li>
        <li>
          <button className="wallet-button" onClick={() => handleNavigate('/Test')}>
            Log In
          </button>
        </li>
      </ul>

      <div className="menubutton1" onClick={handleClick}>
        {click ? <AiOutlineMinus size={25} style={{ color: '#fff' }} /> : <AiOutlineMenu size={25} style={{ color: '#fff' }} />}
      </div>
    </header>
  );
};

export default Header;