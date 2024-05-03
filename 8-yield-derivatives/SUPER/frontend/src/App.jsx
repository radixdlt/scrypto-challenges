import "./App.css";
import HomePage from "./pages/HomePage"
import DAppPage from "./pages/DAppPage"
import DocsPage from "./pages/DocsPage"
import BuySuperPage from "./pages/BuySuperPage";
import OwnerPage from "./pages/DevPage/index.jsx";
import ManageSuperPage from "./pages/ManageSuperPage/index.jsx";

import {BrowserRouter as Router, Routes, Route, Navigate} from 'react-router-dom';
import SecondaryNavbar from "./components/SecondaryNavBar.jsx";

function App() {


    return (
      <Router>
          <Routes>
              <Route path="/" element={<HomePage />} />
              <Route path="DApp" element={<DAppPage/>}/>
              <Route path="Docs" element={<DocsPage/>}/>
              <Route path="/super" element={<BuySuperPage />}>
                  <Route index element={<Navigate replace to="/super/buy" />} />
                  <Route path="buy" element={<BuySuperPage/>}/>
                  <Route path="manage" element={<ManageSuperPage/>}/>
              </Route>
              <Route path="DevsOnly" element={<OwnerPage/>}/>
          </Routes>
      </Router>
  );
}

export default App;
