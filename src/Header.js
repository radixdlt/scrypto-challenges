import React from 'react';

import './Header.css'; // Import CSS for styling

function Header() {
  return (
    <header>
      <nav>
        <ul>
          <li><a href="/" className="nav-link">Home</a></li>
          <li><a href="/about" className="nav-link">About</a></li>
          <li><a href="/contact" className="nav-link">Contact</a></li>
        </ul>
      </nav>
    </header>
  );
}

export default Header;

