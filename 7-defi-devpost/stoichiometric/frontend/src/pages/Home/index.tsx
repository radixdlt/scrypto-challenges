/** @jsxImportSource theme-ui */
import { useContext, useState } from "react";

import { ResponsiveContext } from 'contexts/ResponsiveContext';

import { NavLink } from "react-router-dom";

import { randomIntFromInterval } from "utils/general/generalMaths";

import Button from "components/Button";
import Star from "components/Star";

import bitcoin from "img/bitcoin.png";
import ethereum from "img/ethereum.png";
import styleFunction from "./style";

function Home() {
    const { device } = useContext(ResponsiveContext);

    const [stars, setStars] = useState(Array.from({ length: 5 }, (_, i) => [randomIntFromInterval(0, 1), randomIntFromInterval(10, 90), randomIntFromInterval(10, 90), randomIntFromInterval(0, 1)]));

    const style = styleFunction(device);

    return (
        <div>

            {stars.map((x, index) => {
                return (
                    <Star key={"star" + index} left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"} />
                )
            })}


            <div sx={style.top}>
                <h1 sx={style.beaker}>Stoichiometric<span sx={style.fi}>.beaker.fi</span></h1>
                <div sx={style.right}>
                    {device == "mobile" ? null : <div sx={{ ...style.social, ...style.telegram }} onClick={() => window.location.replace("https://t.me/BeakerFi")} />}
                    {device == "mobile" ? null : <div sx={{ ...style.social, ...style.discord }} onClick={() => window.location.replace("https://discord.com/invite/8CwZkCA7Au")} />}
                    {device == "mobile" ? null : <div sx={{ ...style.social, ...style.twitter }} onClick={() => window.location.replace("https://twitter.com/beaker_fi")} />}
                    {device == "mobile" ? null : <p sx={style.link} onClick={() => window.location.replace("https://github.com/PointSquare/stoichiometric")}>Docs</p>}
                    <NavLink sx={style.navlink} to="/swap">
                        <Button>Enter App</Button>
                    </NavLink>
                </div>
            </div>


            <div sx={style.center}>

                <h1 sx={style.catchphrase}>
                    Don't trust firms, trust <span sx={style.highlight}>debt.</span>
                </h1>
                <h3 sx={style.subtitle}>
                    Use our <span sx={style.highlight}>decentralised stablecoin</span> without trusting <span sx={style.highlight}>unreliable</span> companies
                </h3>

                {device == "mobile" ?
                    <div sx={style.socialRow}>
                        <div sx={{ ...style.social, ...style.telegram }} onClick={() => window.location.replace("https://t.me/BeakerFi")} />
                        <div sx={{ ...style.social, ...style.discord }} onClick={() => window.location.replace("https://discord.com/invite/8CwZkCA7Au")} />
                        <div sx={{ ...style.social, ...style.twitter }} onClick={() => window.location.replace("https://twitter.com/beaker_fi")} />
                    </div>
                    :
                    null
                }

            </div>


            <img sx={style.bitcoin} src={bitcoin} />
            <img sx={style.ethereum} src={ethereum} />

        </div>
    )
}

export default Home;