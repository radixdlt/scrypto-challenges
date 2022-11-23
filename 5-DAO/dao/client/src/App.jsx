import { Routes, Route } from 'react-router-dom';
import './App.css';

// Import Pages
import Dashboard from './pages/Dashboard';
import Marketplace from './pages/Marketplace';
import Lander from './pages/Lander';

// Import Components
import Navbar from './components/Navbar';

function App() {
  return (
    <div className="">
      <Navbar />
      <Routes>
        <Route path="/" element={<Lander />} />
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/marketplace" element={<Marketplace />} />
      </Routes>
    </div>
  );
}

export default App;
