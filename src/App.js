import React, { useState } from 'react';
import './App.css';
import Header from './Header';
import Footer from './Footer';
import Intro from './Intro';
import LogIn from './LogIn';
import Profile from './Profile';
import Team from './Team';

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  const handleLogin = () => {
    setIsLoggedIn(true);
  };

  return (
    <div className="App">
      <header className="App-header">
        <Header />
      </header>
      <div>
        <Intro />
      </div>
      <div>
        <Team />
      </div>
      {/* Conditionally render LoginForm or Profile based on login status */}
      {isLoggedIn ? <Profile /> : <LogIn onLogin={handleLogin} />}
      <Footer />
    </div>
  );
}

export default App;
