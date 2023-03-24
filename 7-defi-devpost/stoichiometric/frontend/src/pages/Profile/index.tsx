/** @jsxImportSource theme-ui */
import { useNavigate } from "react-router-dom";

import { useContext, useState, useEffect } from "react";

import { randomIntFromInterval } from "utils/general/generalMaths";

import Dashboard from "components/Dashboard";
import Star from "components/Star";
import Snackbar from "components/Snackbar";

import { UserContext } from "contexts/UserContext";
import { ResponsiveContext } from "contexts/ResponsiveContext";
import { ThemeContext } from 'contexts/ThemeContext';

import styleFunction from "./style";

import { account } from "types";

function Profile() {

    const navigate = useNavigate();

    const { user, achievements, logoutUser, accountsList, setUser } = useContext(UserContext);
    const { device } = useContext(ResponsiveContext);
    const { themeStyle, toggleTheme } = useContext(ThemeContext);

    const [stars, setStars] = useState(Array.from({ length: 10 }, (_, i) => [randomIntFromInterval(0, 1), randomIntFromInterval(10, 90), randomIntFromInterval(10, 90), randomIntFromInterval(0, 1)]));

    const [accountSelect, setAccountSelect] = useState(false);


    const style = styleFunction(device, themeStyle, accountSelect);

    return (
        <Dashboard page='profile'>
            <Snackbar />

            {stars.map((x, index) => {
                return (
                    <Star key={"star" + index} left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"} />
                )
            })}

            <div sx={style.main}>


                <div sx={style.settings}>

                    <h1>🎨 Theme</h1>
                    <div sx={style.themeRow}>
                        <div sx={style.themeSelector} onClick={() => { toggleTheme() }} />
                    </div>

                </div>


                <div sx={style.profile}>

                    <div sx={style.accountSelectorContainer}>
                        <div sx={style.accountSelectorList}>
                            <h2><div sx={style.close} onClick={() => setAccountSelect(false)} />Select an Account</h2>
                            <div sx={style.accList}>
                                {accountsList.map((account: account, index: number) => {
                                    return (
                                        <div key={"account" + index} sx={style.accChoice} onClick={() => {
                                            setAccountSelect(false);
                                            setUser({ address: account.address, name: account.name })

                                        }}>
                                            <p>{account.address.slice(0, 10) + "..." + account.address.slice(account.address.length - 15, account.address.length)}</p>
                                            <p>{account.name}</p>
                                            <div sx={style.expand} />
                                        </div>)
                                })}
                            </div>
                        </div>
                    </div>

                    <h1>😎 Your profile</h1>
                    <div sx={style.accountSelector} onClick={() => setAccountSelect(true)}>
                        <p>{user.address.slice(0, 10) + "..." + user.address.slice(user.address.length - 15, user.address.length)}</p>
                        <p>{user.name}</p>
                        <div sx={style.expand} />
                    </div>
                    <button onClick={() => {
                        logoutUser();
                        navigate('/');
                    }}>Log out</button>

                </div>


            </div>
        </Dashboard>
    )
}

export default Profile;