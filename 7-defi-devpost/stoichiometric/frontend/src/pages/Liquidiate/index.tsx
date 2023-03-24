/** @jsxImportSource theme-ui */
import { useNavigate } from "react-router-dom";

import { useContext, useState, useEffect } from "react";

import Dashboard from "components/Dashboard";

import { formatToString, randomIntFromInterval } from "utils/general/generalMaths";

import Star from "components/Star";

import Snackbar from "components/Snackbar";

import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";
import { ResponsiveContext } from "contexts/ResponsiveContext";
import styleFunction from "./style";
import { TokensContext } from "contexts/TokensContext";
import { loan } from "types";

import { stable_coin as stable } from "utils/general/constants";
import { liquidate } from "utils/stablecoin/issuerContractCalls";

function Liquidate() {

    const { addAlert } = useContext(SnackbarContext);

    const [stars, setStars] = useState(Array.from({ length: 10 }, (_, i) => [randomIntFromInterval(0, 1), randomIntFromInterval(10, 90), randomIntFromInterval(10, 90), randomIntFromInterval(0, 1)]));


    const { user } = useContext(UserContext);
    const { device, windowSize } = useContext(ResponsiveContext);

    const { loans, pools } = useContext(TokensContext);

    const [liquidateLoading, setLiquidateLoading] = useState(false);

    async function sendLiquidate(withdraw: string, id: string) {
        setLiquidateLoading(true);
        let flag: boolean;
        flag = await liquidate(user.address, withdraw, id);
        if (flag) {
            addAlert("check", "Transaction submitted!");
        } else {
            addAlert("error", "An error occured");
        }
        setLiquidateLoading(false);
    }


    const style = styleFunction(device, liquidateLoading);

    return (
        <Dashboard page='liquidate'>

            <Snackbar />

            {stars.map((x, index) => {
                return (
                    <Star key={"star" + index} left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"} />
                )
            })}


            <div sx={style.main}>

                <div sx={style.lendContainer}>

                    <div sx={style.lendColumn}>

                        {Array.apply(null, Array(Math.ceil(loans.length / (windowSize.width > 1200 ? 3 : device == "mobile" ? 1 : 2)))).map((x, index) => {
                            return (

                                <div key={"loanRow" + index} sx={style.lendRow}>

                                    {loans.slice((windowSize.width > 1200 ? 3 : device == "mobile" ? 1 : 2) * index, (windowSize.width > 1200 ? 3 : device == "mobile" ? 1 : 2) * (index + 1)).map((x: loan, index: number) => {
                                        const cur_price = pools[x.collateral_token.address] ? pools[x.collateral_token.address].min_rate * (1 + pools[x.collateral_token.address].rate_step ** pools[x.collateral_token.address].current_step) : 1;
                                        return (

                                            <div key={"loan" + index} sx={style.lend}>

                                                <p>Collateral <span>{formatToString(x.collateral_amount)} {x.collateral_token.symb} <img src={x.collateral_token.icon_url} /></span></p>
                                                <p>Borrowed <span>{formatToString(x.amount_lent)} {stable.symb} <img src={stable.icon_url} /></span></p>
                                                {cur_price - x.liquidation_price > 0 || !user.address ?
                                                    <div sx={style.barContainer}>

                                                        <div sx={style.bar}>
                                                            <div sx={{ right: `Min(100%, ${(cur_price - x.liquidation_price) / (cur_price / 2) * 100}%)`, background: `${(cur_price - x.liquidation_price) / (cur_price / 2) > .7 ? "green" : (cur_price - x.liquidation_price) / (cur_price / 2) > .3 ? "orange" : "red"}` }} />
                                                        </div>

                                                        <div sx={style.infos}>
                                                            <p>
                                                                Liquidation Price <span>${formatToString(x.liquidation_price)}</span>
                                                            </p>
                                                            <p>
                                                                Current Price <span>${formatToString(cur_price)}</span>
                                                            </p>
                                                        </div>

                                                    </div>
                                                    :
                                                    <button sx={liquidateLoading ? { ...style.swapButton, ...style.swapButtonLoading } : style.swapButton} onClick={() => liquidateLoading ? null : sendLiquidate(x.amount_to_liquidate.toString(), x.id)}>{liquidateLoading ? "" : "Liquidate"}</button>
                                                }

                                            </div>

                                        )
                                    })}

                                </div>

                            )
                        })}
                    </div>

                </div>


            </div>
        </Dashboard>
    )
}

export default Liquidate;