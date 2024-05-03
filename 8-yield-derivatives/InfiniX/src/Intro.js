import React from "react";
import CoverImg from "./y.jpg";

import "./Main.css";


const Blankpage = () => {
  return (
    <div className="blank">
      <div className="wallpaper">
        <img className="blank-img" src={CoverImg} alt="Coverimg" />
        <div className="text-container">
        <h1 style={{ fontSize: '12em' }}> ♾️</h1>
          <h4>Instant Insurance, Infinite Possibilities </h4>
        </div>
      </div>
    </div>
  );
};

export default Blankpage;