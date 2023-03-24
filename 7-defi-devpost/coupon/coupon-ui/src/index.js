import ReactDOM from "react-dom";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import GenerateGiftCode from "./pages/GenerateGiftCode";
import RedeemGiftCode from "./pages/RedeemGiftCode";
import Layout from "./pages/Layout";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<GenerateGiftCode />} />
          <Route path="redeem" element={<RedeemGiftCode />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));