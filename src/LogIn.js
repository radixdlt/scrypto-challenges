import React, { useState } from "react";
import axios from "axios";

const LoginForm = ({ onLogin }) => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState(null);

  const handleEmailChange = (e) => setEmail(e.target.value);
  const handlePasswordChange = (e) => setPassword(e.target.value);

  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      const response = await axios.post("http://localhost:5001/api/login", {
        email,
        password,
      }, {
        headers: {
          'Content-Type': 'application/json'
        }
      });
      console.log(response.data);
      // Call the onLogin function passed from App.js to update login status
      onLogin();
    } catch (error) {
      setError("Invalid email or password");
      console.error("Login error:", error);
    }
  };

  return (
    <div className="wallet_container" style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', padding: '20px' }}>
      <div className="send_xrp_container">
        <h2>Login Here! </h2>
        <form onSubmit={handleSubmit}>
          <div style={{ marginBottom: '10px' }}>
            <label>Email:</label>
            <input
              type="email"
              placeholder="Hint: user2@example.com"
              value={email}
              onChange={handleEmailChange}
              required
            />
          </div>
          <div style={{ marginBottom: '10px' }}>
            <label>Password:</label>
            <input
              type="password"
              placeholder="Hint: password2"
              value={password}
              onChange={handlePasswordChange}
              required
            />
          </div>

          <button className="wallet-button" type="submit">Login</button>
        </form>
        {error && <p>{error}</p>}
      </div>
    </div>
  );
};

export default LoginForm;
