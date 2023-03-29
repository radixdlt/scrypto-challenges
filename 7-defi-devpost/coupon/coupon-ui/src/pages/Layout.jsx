import React from "react";
import {Outlet} from "react-router-dom";
import '../style/Layout.css';

const Layout = () => {
  return (
    <>
      <Outlet />
    </>
  );
};

export default Layout;