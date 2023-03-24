/** @jsxImportSource theme-ui */

import { useContext, useEffect, useState, useRef } from "react";
import { useSearchParams } from "react-router-dom";

import { ResponsiveContext } from "contexts/ResponsiveContext";
import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";

import Star from "components/Star";

import Dashboard from "components/Dashboard";
import ConnectWallet2 from "components/ConnectWalletLarge";

import Snackbar from "components/Snackbar";
import { TokensContext } from "contexts/TokensContext";

import { formatToString, formatToString2, randomIntFromInterval } from "utils/general/generalMaths";


import { stable_coin as stable, stable_coin, token_default } from "utils/general/constants";

import styleFunction from "./style";

import { token, position } from "types";

function Card() {

    const [stars, setStars] = useState(Array.from({ length: 10 }, (_, i) => [randomIntFromInterval(0, 1), randomIntFromInterval(10, 90), randomIntFromInterval(10, 90), randomIntFromInterval(0, 1)]));

    const { addAlert } = useContext(SnackbarContext);

    const { device } = useContext(ResponsiveContext);

    const { tokens } = useContext(TokensContext);

    const { user, tokensOwned, setNbTokens, positions, voterCard } = useContext(UserContext);

    const [tokensList, setTokensList] = useState(tokens);

    const [token1Owned, setToken1Owned] = useState<"?" | number>("?");

    const [sent, setSent] = useState<number>(0);
    const [get, setGet] = useState<number>(0);

    const [token1Select, setToken1Select] = useState(false);

    const [token1, setToken1] = useState<token>(stable);

    const [position, setPosition] = useState<position>({
        token: null,
        liquidity: 0,
        price_x: 1,
        value_locked: '?',
        x_fees: '?',
        y_fees: '?',
        nfIdValue: "-1",
        id: "-1"
    })

    const [isToken, setIsToken] = useState<boolean>(true);

    const [search, setSearch] = useState("");

    const [swapLoading, setSwapLoading] = useState(false);


    function resetValues() {
        setSent(0);
        setGet(0);
    }

    function resetSelect() {
        setSearch('');
        setToken1Select(false);
    }



    const sentChange = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                resetValues();
                return
            }
            if (s.includes(".")) {
                setSent(s);
                if (isToken) setGet(parseFloat(s));
            } else {
                setSent(parseFloat(s));
                if (isToken) setGet(parseFloat(s));
            }
        }
    }

    useEffect(() => {
        const n = tokensOwned[token1.address];
        if (n == "undefined") setToken1Owned(0);
        else setToken1Owned(parseFloat(n));
    }, [tokensOwned, token1])

    function getSearch(list: any[]) {
        return list.filter((x: position) => {
            var flag = (search.length == 0);
            for (const word of search.split(' ')) {
                if (word.length > 0 && x.token) flag = flag || x.token.name.toLowerCase().includes(word) || x.token.symb.toLowerCase().includes(word)
            }
            return flag
        })
    }

    const searchChange = (event: any) => {
        setSearch(event.target.value.toLowerCase());
    }

    useEffect(() => {
        setTokensList(getSearch(positions));
    }, [tokens, search, positions])

    function selectToken(token: token) {
        setToken1(token)
        resetSelect();
        resetValues();
    }

    function selectPosition(position: position) {
        setPosition(position);
        resetValues();
        if (position.value_locked == "?") setGet(0);
        else {
            setGet(position.value_locked);
        }
        resetSelect();
    }

    async function sendSwap() {
        setSwapLoading(true);
        let flag: boolean = true;
        setNbTokens();
        resetValues();
        if (flag) {
            addAlert("check", "Transaction submitted!");
        } else {
            addAlert("error", "An error occured");
        }
        setSwapLoading(false);
    }



    const style = styleFunction(device, swapLoading, token1Select);


    return (
        <Dashboard page="card">

            <Snackbar />

            {stars.map((x, index) => {
                return (
                    <Star key={"star" + index} left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"} />
                )
            })}

            <div sx={style.main}>

                <div sx={style.top}>

                    <div sx={style.container}>

                        <div sx={style.swapZone}>


                            <h1>💳 Add Voting Power</h1>


                            {isToken ?
                                <div sx={style.inputBar}>
                                    <input type="text" id="send" required={true} placeholder=" " autoComplete="off" onChange={sentChange} value={sent} />
                                    <label htmlFor="send">{user.address ? `You have ${token1Owned == "?" ? "?" : isNaN(token1Owned) ? 0 : formatToString(token1Owned)} ${token1.symb}` : "You lock"}</label>
                                    <div sx={style.token} onClick={() => setToken1Select(true)}>
                                        <img src={token1.icon_url} />
                                        <p>{token1.symb}</p>
                                        <div sx={style.expand} />
                                    </div>
                                </div>
                                :
                                <div sx={style.inputBar}>
                                    <input type="text" id="send" required={true} placeholder=" " autoComplete="off" disabled value={1} />
                                    <label htmlFor="send">You lock</label>
                                    <div sx={style.token3} onClick={() => setToken1Select(true)}>
                                        <img src={position.token ? position.token.icon_url : token_default.icon_url} />
                                        <p>Position</p>
                                        <div sx={style.expand} />
                                    </div>
                                </div>
                            }


                            {isToken ?
                                <span sx={style.tokenAddress}><span>Token Address</span>{token1.address.slice(0, 5) + "..." + token1.address.slice(token1.address.length - 10, token1.address.length)}</span>
                                :
                                <span sx={style.tokenAddress}><span>Position ID</span>{position.id}</span>
                            }


                            <div sx={style.inputBar}>
                                <input type="text" id="get" required={true} placeholder=" " autoComplete="off" disabled value={get} />
                                <label htmlFor="get">You get</label>
                                <div sx={style.token2}>
                                    <p>Voting Power</p>
                                </div>
                            </div>


                            <div sx={style.swapInfos}>
                                <span sx={style.swapInfoMain}><span>Lock</span><div>{isToken ? typeof (sent) == "string" ? formatToString(parseFloat(sent)) : formatToString(sent) : 1} {isToken ? token1.symb : "Position"}<div />{typeof (get) == "string" ? formatToString(parseFloat(get)) : formatToString(get)} Voting Power</div></span>
                                <span sx={style.swapInfo}><span>Total Voting Power</span>{voterCard.voting_power + get}</span>
                            </div>

                            {user.address ?
                                <div>
                                    <button sx={swapLoading ? { ...style.swapButton, ...style.swapButtonLoading } : style.swapButton} onClick={() => swapLoading ? null : sendSwap()}>{swapLoading ? "" : "Lock"}</button>
                                    <button sx={swapLoading ? { ...style.swapButton, ...style.swapButtonLoading } : style.swapButton} onClick={() => swapLoading ? null : sendSwap()}>{swapLoading ? "" : "Unlock All"}</button>
                                </div>
                                :
                                <ConnectWallet2 />
                            }


                            <div sx={style.selectToken}>

                                <h2><div sx={style.close} onClick={resetSelect} />Select Currency or Position</h2>

                                <div sx={style.inputBar}>
                                    <input type="text" id="search" required={true} placeholder=" " autoComplete="off" onChange={searchChange} value={search} />
                                    <label htmlFor="search">Search for a position</label>
                                </div>

                                <div sx={style.tokensList}>

                                    {[stable_coin].map((token: token, index: number) => {
                                        return (
                                            <div key={"token" + index} sx={style.tokenChoice} onClick={() => {
                                                selectToken(token);
                                                setIsToken(true);
                                            }}>
                                                <img src={token.icon_url} />
                                                <p>{token.name}<span>{token.symb}</span></p>
                                            </div>
                                        )
                                    })}

                                    {tokensList.map((position: position, index: number) => {
                                        if (position.token)
                                            return (
                                                <div key={"token" + index} sx={style.tokenChoice} onClick={() => {
                                                    selectPosition(position);
                                                    setIsToken(false);
                                                }}>
                                                    <img src={position.token.icon_url} />
                                                    <p>{position.token.name}<span>{position.token.symb}</span></p>
                                                </div>
                                            ); else return null;
                                    })}

                                </div>

                            </div>


                        </div>

                    </div>

                </div>

            </div>

        </Dashboard>
    )
}

export default Card;