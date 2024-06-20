import "./App.css";

import {BrowserRouter as Router, Routes, Route, Navigate} from 'react-router-dom';

import HomePage from "./pages/HomePage/index.jsx"
import DocsPage from "./pages/DocsPage/index.jsx"
import BuySuperPage from "./pages/BuySuperPage/index.jsx";
import OwnerPage from "./pages/OwnerPage/index.jsx";
import SuperPage from "./pages/SuperPage/index.jsx";
import ManageSuperPageV2 from "./pages/ManageSuperPage/index.jsx";

function App() {

    return (
        <Router>
            <Routes>

                {/* Home page route */}
                <Route path="/" element={<HomePage />} />


                {/* Super page with nested routes */}
                <Route path="/super" element={<SuperPage />}>
                  {/* Default route redirects to /super/buy */}
                  <Route index element={<Navigate to="/super/buy" />} />
                  {/* Buy Super page route */}
                  <Route path="buy" element={<BuySuperPage />} />
                  {/* Manage Super page route */}
                  <Route path="manage" element={<ManageSuperPageV2/>}/>
                </Route>

                {/* Owner/Developer page route */}
                <Route path="DevsOnly" element={<OwnerPage/>}/>

                {/* Docs page route */}
                <Route path="Docs" element={<DocsPage/>}/>

            </Routes>
        </Router>
  );
}

export default App;
