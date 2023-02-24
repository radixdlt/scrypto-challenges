import {NavLink} from "react-router-dom"
import Connect from "./Connect"

export default function Header(props) {
    return (
        <div className="navbar bg-base-100">
            <div className="navbar-start">
                <div className="dropdown">
                    <label tabIndex={0} className="btn btn-ghost lg:hidden">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24"
                             stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                                  d="M4 6h16M4 12h8m-8 6h16"/>
                        </svg>
                    </label>
                    <ul tabIndex={0}
                        className="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
                        <li>
                            <NavLink to="/">
                                Home
                            </NavLink>
                        </li>
                        <li>
                            <NavLink to="/">
                                Invest
                            </NavLink>
                        </li>
                        <li>
                            <NavLink to="/">
                                Your Collection
                            </NavLink>
                        </li>
                    </ul>
                </div>
                <a className="btn btn-ghost normal-case text-xl">Fraction</a>
            </div>
            <div className="navbar-center hidden lg:flex">
                <ul tabIndex={0} className="menu gap-2 horizontal">
                    <li>
                        <NavLink to="/" className="rounded-lg">
                            Home
                        </NavLink>
                    </li>
                    <li>
                        <NavLink to="/invest" className="rounded-lg">
                            Invest
                        </NavLink>
                    </li>
                    <li>
                        <NavLink to="/collection" className="rounded-lg">
                            Your Collection
                        </NavLink>
                    </li>
                </ul>
            </div>
            <div className="navbar-end">
                <Connect/>
            </div>
        </div>
    )
}