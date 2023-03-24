import {useContext} from "react"
import {BlockContext} from "./Context"


export default function Connect() {
    const {address, connect} = useContext(BlockContext)

    return (
        <button className="btn btn-primary" onClick={connect}>
            {(address !== null) ? "0x..." + address.substring(address.length - 6, address.length) : "Connect"}
        </button>
    )
}