import {useState, useContext} from "react"
import {BlockContext} from "./Context"
// import {BigNumber} from "ethers"

export default function Invest() {
    const {buy} = useContext(BlockContext)
    const [amount, setAmount] = useState(0)

    return (
        <div className="w-full h-full max-w-2xl flex justify-between items-center gap-4 py-10">
            <div className="h-full flex-1 flex flex-col justify-center gap-4">
                <h1 className="bg-base-100 text-center px-6 py-4 rounded-lg">Newest Asset</h1>

                <div className="bg-base-100">
                    <img src={require("../data/porsche.png")} className="w-full" alt="Lambo"/>
                </div>

                <div className="flex relative">
                    <input type="text" placeholder="0" className="input input-bordered w-full pr-16"
                    onChange={(e) => setAmount(parseInt(e.target.value))}/>
                    <button className="btn btn-primary absolute top-0 right-0 rounded-l-none"
                    onClick={() => buy(amount)}
                    >Invest</button>
                </div>
            </div>

            <div className="h-full flex items-center">
                <div className="w-52 flex flex-col justify-center items-center gap-4">
                    <div className="w-full bg-base-100 rounded-lg p-4">
                        <p className="font-medium">12%</p>
                        <p className="text-sm text-primary/50">Expected Performance</p>
                    </div>
                    <div className="w-full bg-base-100 rounded-lg p-4">
                        <p className="font-medium">1000 XRD</p>
                        <p className="text-sm text-primary/50">Current Value</p>
                    </div>
                    <div className="w-full bg-base-100 rounded-lg p-4">
                        <p className="font-medium">10 XRD</p>
                        <p className="text-sm text-primary/50">per fractional NFT</p>
                    </div>
                </div>
            </div>
        </div>
    )
}