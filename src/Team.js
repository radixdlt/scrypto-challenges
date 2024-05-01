import React from "react";
import { Link } from "react-router-dom";
import bannerImg from ".";
import "./AboutUs.css";
import Container from "react-bootstrap/Container";
/*import logoftp from "./img/fin.jpg";
import avatar from "./img/fin.jpg";
import abdur from "./img/fin.jpg";*/
import rebecca from "./y.jpg";
import abdurr from "./y.jpg";


const About = () => {
  return (
    <div className="row text-center">
    <div className="container">
      <div className="aboutUs">
       
      <div>
  <h2 style={{ marginBottom: "10px" }}>Our Mission</h2> {/* Add margin-bottom */}
  <p style={{ marginBottom: "20px" }}> {/* Add margin-bottom */}
 InfiniX Enhances DeFi security with customizable loss limits and instant fund settlements, leveraging our unique parametric insurance model to match personal risk preferences and ensure robust investment protection.

</p>
  <hr />
</div>
        
        <h2>Our Team</h2>
        <div className="grid-container">
          <div classname="grid-item">
            <img className="banner-img" src={rebecca} alt="Bannerimg" />
            <h4 className="font-weight-bold dark-grey-text mt-4">Abdur Razzak</h4>
            <h6 className="font-weight-bold blue-text my-3">
              Developer
            </h6>
            <p className="font-weight-normal dark-grey-text"></p>
          </div>
          <div classname="grid-item">
            <div className="testimonial mb-5">
              <img className="banner-img" src={abdurr} alt="Bannerimg" />
              <h4 className="font-weight-bold dark-grey-text mt-4">Kishan Marsonia
</h4>
              <h6 className="font-weight-bold blue-text my-3">
                FinTech Expert
              </h6>
              <p className="font-weight-normal dark-grey-text"></p>
            </div>
          </div>
         
          
         
          
        </div>
      </div>
    </div>
    </div>
  );
};

export default About;