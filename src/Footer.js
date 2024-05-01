import React from 'react';
import { FaHome, FaPhone, FaMailBulk, FaFacebook, FaTwitter, FaInstagram } from 'react-icons/fa'; // Importing the necessary icons
import "./Main.css"; // Importing the CSS file

const Footer = () => {
  return (
    <div className="footer"> {/* Applying the footer class */}
      <div className="footer-container">
        <div className="left">
          <div className="location">
            <FaHome size={20} style={{ color: "#fff", marginRight: "2rem" }} />
            <p>ArtSect Gallery, London</p>
          </div>
          <div className="phone">
            <p>
              <FaPhone
                size={20}
                style={{ color: "#fff", marginRight: "2rem" }}
              />
              07438282689
            </p>
          </div>
          <div className="email">
            <p>
              <FaMailBulk
                size={20}
                style={{ color: "#fff", marginRight: "2rem" }}
              />
              InfiniX.co.uk
            </p>
          </div>
        </div>

        <div className="right">
          <h4>About</h4>
          <p>
          InfiniX offers customizable loss limits and instant fund settlements, leveraging our unique parametric insurance model.</p>
          <div className="socialmedia">
            <FaFacebook
              size={30}
              style={{ color: "#fff", marginRight: "1rem" }}
            />
            <FaTwitter
              size={30}
              style={{ color: "#fff", marginRight: "1rem" }}
            />
            <FaInstagram
              size={30}
              style={{ color: "#fff", marginRight: "1rem" }}
            />
          </div>
        </div>
      </div>
    </div>
  )
}

export default Footer;