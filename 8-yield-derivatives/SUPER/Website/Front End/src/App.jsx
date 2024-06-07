import "./App.css";
import HomePage from "./pages/HomePage/index.jsx"
import DocsPage from "./pages/DocsPage/index.jsx"
import BuySuperPage from "./pages/BuySuperPage/index.jsx";
import OwnerPage from "./pages/DevPage/index.jsx";

import {BrowserRouter as Router, Routes, Route, Navigate} from 'react-router-dom';
import SuperPage from "./pages/SuperPage/index.jsx";
import ManageSuperPageV2 from "./pages/ManageSuperPageV2/index.jsx";

function App() {


    return (
      <Router>
          <Routes>
              <Route path="/" element={<HomePage />} />
              <Route path="Docs" element={<DocsPage/>}/>
              <Route path="/super" element={<SuperPage />}>
                  <Route index element={<Navigate to="/super/buy" />} />
                  <Route path="buy" element={<BuySuperPage />} />
                  <Route path="manage" element={<ManageSuperPageV2/>}/>
              </Route>
              <Route path="DevsOnly" element={<OwnerPage/>}/>
          </Routes>
      </Router>
  );
}

export default App;
