import React from "react";
import { BrowserRouter, Route, Link } from "react-router-dom";

function Navbar() {
  return (
    <nav>
      <ul>
        <li>
          <Link to="/">Home</Link>
        </li>
        <li>
          <Link to="/Issue">Issue</Link>
        </li>
        <li>
          <Link to="/Sell">Sell</Link>
        </li>
        <li>
          <Link to="/Buy">Buy</Link>
        </li>
      </ul>
    </nav>
  );
}

export default Navbar;