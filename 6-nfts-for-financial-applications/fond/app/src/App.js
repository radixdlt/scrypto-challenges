import './App.css'
import {useState} from "react"
import {Route, Routes} from "react-router-dom"
import Header from "./components/Header"
import Home from "./components/Home"
import Invest from "./components/Invest"
import Investments from "./components/Investments"


function App() {
    const [address, setAddress] = useState(null)

    return (
        <div className="App h-screen flex flex-col">
            <Header address={address} setAddress={setAddress}/>
            <div className="w-full flex flex-1 justify-center items-center bg-base-200 p-4">
                <Routes>
                    <Route path="/" element={<Home/>}/>
                    <Route path="/invest" element={<Invest/>}/>
                    <Route path="/collection" element={<Investments/>}/>
                </Routes>
            </div>
        </div>
    )
}

export default App;
