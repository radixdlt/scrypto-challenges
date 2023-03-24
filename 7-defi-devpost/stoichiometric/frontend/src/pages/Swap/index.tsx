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

import { formatToString, randomIntFromInterval } from "utils/general/generalMaths";


import { stable_coin as stable, token_default } from "utils/general/constants";

import { swap_direct, swap_indirect } from "../../utils/dex/routerContractCalls";
import styleFunction from "./style";

import { token, step } from "types";

function Swap() {

    let [searchParams, setSearchParams] = useSearchParams();

    const [stars, setStars] = useState(Array.from({ length: 10 }, (_, i) => [randomIntFromInterval(0, 1), randomIntFromInterval(10, 90), randomIntFromInterval(10, 90), randomIntFromInterval(0, 1)]));

    const { addAlert } = useContext(SnackbarContext);

    const [alert, setAlert] = useState<boolean>(false);

    const { device } = useContext(ResponsiveContext);

    const { tokens, pools } = useContext(TokensContext);

    const { user, tokensOwned, setNbTokens } = useContext(UserContext);

    const [tokensList, setTokensList] = useState(tokens);

    const [token1Owned, setToken1Owned] = useState<"?" | number>("?");

    const [price, setPrice] = useState<number>(0);

    const [sent, setSent] = useState<number>(0);
    const [get, setGet] = useState<number>(0);
    const [intermediateGet, setIntermediateGet] = useState<number>(0);

    const [token1, setToken1] = useState(stable);
    const [token2, setToken2] = useState(token_default);

    const [token1Select, setToken1Select] = useState(false);
    const [token2Select, setToken2Select] = useState(false);

    const [swapLoading, setSwapLoading] = useState(false);

    const [search, setSearch] = useState("");

    const token1AddressRef = useRef(token1.address)
    token1AddressRef.current = token1.address

    const token2AddressRef = useRef(token2.address)
    token2AddressRef.current = token2.address

    function resetValues() {
        setSent(0);
        setGet(0);
        setIntermediateGet(0);
    }

    useEffect(() => {
        var tk1 = searchParams.get('tk1');
        var tk2 = searchParams.get('tk2');

        if (!tk1 || !tk2) {
            setSearchParams({ tk1: stable.symb, tk2: token_default.symb })
        }
    }, [])

    useEffect(() => {
        if (tokens) {
            var tk1 = searchParams.get('tk1');
            var tk2 = searchParams.get('tk2');

            if (tk1 && tk2) {
                tk1 = tk1.toLowerCase();
                tk2 = tk2.toLowerCase();
            }

            if (tk1 != tk2 && tokens.map((x: token) => x.symb.toLowerCase()).includes(tk1) && tokens.map((x: token) => x.symb.toLowerCase()).includes(tk2)) {
                var tok1 = tokens.filter((x: token) => x.symb.toLowerCase() == tk1)[0]
                var tok2 = tokens.filter((x: token) => x.symb.toLowerCase() == tk2)[0]
                setToken1({ name: tok1!.name, symb: tok1!.symb, address: tok1!.address, icon_url: tok1!.icon_url });
                setToken2({ name: tok2!.name, symb: tok2!.symb, address: tok2!.address, icon_url: tok2!.icon_url });
                setSearchParams({ tk1: tk1!.toUpperCase(), tk2: tk2!.toUpperCase() })
            } else {
                setToken1(stable);
                setToken2(token_default);
                setSearchParams({ tk1: stable.symb, tk2: token_default.symb })
            }
        }

    }, [tokens])

    function findIndex(n: number, list: step[]) {
        for (var i = 0; i < list.length; ++i) if (list[i].step_id == n) return i
        return -1;
    }

    function calculateGet(n: number) {
        if (token1.address == stable.address) {
            const pool = pools[token2.address];
            var temp = n;
            var recieved = 0;
            actualPool = pool["current_step"];
            index = findIndex(actualPool, pool["steps"]);

            while (temp > 0 && index < pool["steps"].length) {
                var temp2 = temp;
                temp = temp - Math.min(pool["steps"][index].other_token_amount * pool["steps"][index].rate, temp);
                recieved = recieved + Math.min(pool["steps"][index].other_token_amount, temp2 / pool["steps"][index].rate);
                index = index + 1;
            }
            return recieved;
        } else if (token2.address == stable.address) {
            const pool = pools[token1.address];
            var temp = n;
            var recieved = 0;
            var actualPool = pool["current_step"];
            var index = findIndex(actualPool, pool["steps"]);

            while (temp > 0 && index >= 0) {
                var temp2 = temp;
                temp = temp - Math.min(pool["steps"][index].stablecoin_amount / pool["steps"][index].rate, temp);
                recieved = recieved + Math.min(pool["steps"][index].stablecoin_amount, temp2 * pool["steps"][index].rate);
                index = index - 1;
            }
            return recieved;
        } else {
            const pool1 = pools[token1.address];
            var temp = n;
            var recieved = 0;
            var actualPool = pool1["current_step"];
            var index = findIndex(actualPool, pool1["steps"]);

            while (temp > 0 && index >= 0) {
                var temp2 = temp;
                temp = temp - Math.min(pool1["steps"][index].stablecoin_amount / pool1["steps"][index].rate, temp);
                recieved = recieved + Math.min(pool1["steps"][index].stablecoin_amount, temp2 * pool1["steps"][index].rate);
                index = index - 1;
            }

            const pool2 = pools[token2.address];
            var recieved2 = 0;
            actualPool = pool2["current_step"];
            index = findIndex(actualPool, pool2["steps"]);

            while (recieved > 0 && index < pool2["steps"].length) {
                var recieved3 = recieved;
                recieved = recieved - Math.min(pool2["steps"][index].other_token_amount * pool2["steps"][index].rate, recieved);
                recieved2 = recieved2 + Math.min(pool2["steps"][index].other_token_amount, recieved3 / pool2["steps"][index].rate);
                index = index + 1;
            }
            return recieved2;
        }
    }

    function needAlert(n: number) {
        if (token1.address == stable.address) {
            const pool = pools[token2.address];
            if (!pool) return 0;
            var temp = n;
            var recieved = 0;
            actualPool = pool["current_step"];

            index = findIndex(actualPool, pool["steps"]);

            while (temp > 0 && index < pool["steps"].length) {
                var temp2 = temp;
                temp = temp - Math.min(pool["steps"][index].other_token_amount * pool["steps"][index].rate, temp);
                recieved = recieved + Math.min(pool["steps"][index].other_token_amount, temp2 / pool["steps"][index].rate);
                index = index + 1;
            }
            if (temp > 0) setAlert(true); else setAlert(false);
        } else if (token2.address == stable.address) {
            const pool = pools[token1.address];
            if (!pool) return 0;
            var temp = n;
            var recieved = 0;
            var actualPool = pool["current_step"];
            var index = findIndex(actualPool, pool["steps"]);

            while (temp > 0 && index >= 0) {
                var temp2 = temp;
                temp = temp - Math.min(pool["steps"][index].stablecoin_amount / pool["steps"][index].rate, temp);
                recieved = recieved + Math.min(pool["steps"][index].stablecoin_amount, temp2 * pool["steps"][index].rate);
                index = index - 1;
            }
            if (temp > 0) setAlert(true); else setAlert(false);
        } else {
            const pool1 = pools[token1.address];
            if (!pool1) return 0;
            var temp = n;
            var recieved = 0;
            var actualPool = pool1["current_step"];
            var index = findIndex(actualPool, pool1["steps"]);

            while (temp > 0 && index >= 0) {
                var temp2 = temp;
                temp = temp - Math.min(pool1["steps"][index].stablecoin_amount / pool1["steps"][index].rate, temp);
                recieved = recieved + Math.min(pool1["steps"][index].stablecoin_amount, temp2 * pool1["steps"][index].rate);
                index = index - 1;
            }

            const pool2 = pools[token2.address];
            if (!pool2) return 0;
            var recieved2 = 0;
            actualPool = pool2["current_step"];
            index = findIndex(actualPool, pool2["steps"]);
            while (recieved > 0 && index < pool2["steps"].length) {
                var recieved3 = recieved;
                recieved = recieved - Math.min(pool2["steps"][index].other_token_amount * pool2["steps"][index].rate, recieved);
                recieved2 = recieved2 + Math.min(pool2["steps"][index].other_token_amount, recieved3 / pool2["steps"][index].rate);
                index = index + 1;
            }
            if (temp > 0 || recieved > 0) setAlert(true); else setAlert(false);
        }
    }

    useEffect(() => {
        needAlert(sent)
    }, [sent])

    function calculateIntermediate(n: number) {
        const pool1 = pools[token1.address];
        if (!pool1) return 0
        var temp = n;
        var recieved = 0;
        var actualPool = pool1["current_step"];
        var index = findIndex(actualPool, pool1["steps"]);
        while (temp > 0 && index >= 0) {
            var temp2 = temp;
            temp = temp - Math.min(pool1["steps"][index].stablecoin_amount * pool1["steps"][index].rate, temp);
            recieved = recieved + Math.min(pool1["steps"][index].stablecoin_amount, temp2 / pool1["steps"][index].rate);
            index = index - 1;
        }
        return recieved;
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
                var x = calculateGet(parseFloat(s));
                var y = calculateIntermediate(parseFloat(s));
                if (isNaN(x)) { setGet(0); setIntermediateGet(0); }
                else { setGet(x); setIntermediateGet(y); }
            } else {
                setSent(parseFloat(s));
                var x = calculateGet(parseFloat(s));
                var y = calculateIntermediate(parseFloat(s));
                if (isNaN(x)) { setGet(0); setIntermediateGet(0); }
                else { setGet(x); setIntermediateGet(y); }
            }
        }
    }

    useEffect(() => {
        async function getPoolInfos() {
            if (token1.address == stable.address) {
                const pool = pools[token2.address];
                if (!pool) return;

                var actualPool = pool["current_step"];
                var index = findIndex(actualPool, pool["steps"]);

                if (pool["steps"][index].rate > 0) setPrice(1 / pool["steps"][index].rate);
                else setPrice(0);
            } else if (token2.address == stable.address) {
                const pool = pools[token1.address];
                if (!pool) return;

                var actualPool = pool["current_step"];
                var index = findIndex(actualPool, pool["steps"]);

                setPrice(pool["steps"][index].rate);
            } else {
                const pool1 = pools[token1.address];
                if (!pool1) return;

                var actualPool = pool1["current_step"];
                var index = findIndex(actualPool, pool1["steps"]);

                const pool2 = pools[token1.address];
                if (!pool2) return;

                var actualPool = pool2["current_step"];
                var index = findIndex(actualPool, pool2["steps"]);

                if (pool1["steps"][index].rate > 0) setPrice(pool2["steps"][index].rate / pool1["steps"][index].rate);
                else setPrice(0);
            }
        }
        getPoolInfos();
    }, [token1, token2, tokensOwned, pools])

    useEffect(() => {
        const n = tokensOwned[token1.address];
        if (n == "undefined") setToken1Owned(0);
        else setToken1Owned(parseFloat(n));
    }, [tokensOwned, token1])

    function invert() {
        const temp = { ...token2 };
        setToken2(token1);
        setToken1(temp);
        var tk1 = searchParams.get('tk1')!.toUpperCase();
        var tk2 = searchParams.get('tk2')!.toUpperCase();
        setSearchParams({ tk1: tk2, tk2: tk1 })
    }

    useEffect(() => {
        resetValues();
    }, [token1, token2])

    function resetSelect() {
        setSearch('');
        setToken1Select(false);
        setToken2Select(false);
    }

    function selectToken(token: token) {
        if (token1Select) {
            if (token.address == token2.address) {
                invert()
            }
            else {
                setToken1(token)
                var tk2 = searchParams.get('tk2')!.toUpperCase();
                setSearchParams({ tk1: token.symb.toUpperCase(), tk2: tk2 })
            }
        }
        if (token2Select) {
            if (token.address == token1.address) {
                invert()
            }
            else {
                setToken2(token);
                var tk1 = searchParams.get('tk1')!.toUpperCase();
                setSearchParams({ tk1: tk1, tk2: token.symb.toUpperCase() })
            }
        }
        resetSelect();
        resetValues();
    }

    function getSearch(list: any[]) {
        return list.filter(x => {
            var flag = (search.length == 0);
            for (const word of search.split(' ')) {
                if (word.length > 0) flag = flag || x.name.toLowerCase().includes(word) || x.symb.toLowerCase().includes(word)
            }
            return flag
        })
    }

    const searchChange = (event: any) => {
        setSearch(event.target.value.toLowerCase());
    }

    useEffect(() => {
        setTokensList(getSearch(tokens));
    }, [tokens, search])

    async function sendSwap() {
        setSwapLoading(true);
        let flag: boolean;
        if (token1.address != stable.address && token2.address != stable.address) flag = await swap_indirect(user.address, token1.address, token2.address, sent.toString());
        else flag = await swap_direct(user.address, token1.address, token2.address, sent.toString())
        setNbTokens();
        resetValues();
        if (flag) {
            addAlert("check", "Transaction submitted!");
        } else {
            addAlert("error", "An error occured");
        }
        setSwapLoading(false);
    }



    const style = styleFunction(device, swapLoading, token1Select, token2Select);


    return (
        <Dashboard page="swap">
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

                            <h1>🏛 Swap Tokens</h1>

                            {alert ?
                                <div sx={style.alert}>
                                    <p>There is not enough token in the pool for you to swap everything</p>
                                </div>
                                :
                                null
                            }

                            <div sx={style.inputBar}>
                                <input type="text" id="send" required={true} placeholder=" " autoComplete="off" onChange={sentChange} value={sent} />
                                <label htmlFor="send">{user.address ? `You have ${token1Owned == "?" ? "?" : isNaN(token1Owned) ? 0 : formatToString(token1Owned)} ${token1.symb}` : "You send"}</label>
                                <div sx={style.token} onClick={() => setToken1Select(true)}>
                                    <img src={token1.icon_url} />
                                    <p>{token1.symb}</p>
                                    <div sx={style.expand} />
                                </div>
                            </div>

                            <span sx={style.tokenAddress}><span>Token Address</span>{token1.address.slice(0, 5) + "..." + token1.address.slice(token1.address.length - 10, token1.address.length)}</span>

                            {(token1.address != stable.address && token2.address != stable.address) ?

                                <div sx={style.stableBarContainer}>
                                    <div sx={style.swapIcon2} onClick={invert} />
                                    <div sx={style.stableBar}>
                                        <div sx={style.inputBar}>
                                            <input type="text" id="get" required={true} placeholder=" " autoComplete="off" disabled value={intermediateGet} />
                                            <label htmlFor="get">{user.address ? `Intermediate transaction` : "You get"}</label>
                                            <div sx={style.token2}>
                                                <img src={stable.icon_url} />
                                                <p>{stable.symb}</p>
                                            </div>
                                        </div>
                                        <span sx={style.tokenAddress}><span>Token Address</span>{stable.address.slice(0, 5) + "..." + stable.address.slice(stable.address.length - 10, stable.address.length)}</span>
                                    </div>
                                </div>
                                :
                                <div sx={style.swapIcon} onClick={invert} />

                            }

                            <div sx={style.inputBar}>
                                <input type="text" id="get" required={true} placeholder=" " autoComplete="off" disabled value={get} />
                                <label htmlFor="get">You get</label>
                                <div sx={style.token} onClick={() => setToken2Select(true)}>
                                    <img src={token2.icon_url} />
                                    <p>{token2.symb}</p>
                                    <div sx={style.expand} />
                                </div>
                            </div>

                            <span sx={style.tokenAddress}><span>Token Address</span>{token2.address.slice(0, 5) + "..." + token2.address.slice(token2.address.length - 10, token2.address.length)}</span>
                            <div sx={style.swapInfos}>
                                <span sx={style.swapInfoMain}><span>Purchase</span><div>{typeof (sent) == "string" ? formatToString(parseFloat(sent)) : formatToString(sent)} {token1.symb}<div />{typeof (get) == "string" ? formatToString(parseFloat(get)) : formatToString(get)} {token2.symb}</div></span>
                                <span sx={style.swapInfo}><span>Price</span>1 {token1.symb} = {price == 0 ? "?" : sent == 0 ? formatToString(price) : formatToString(get / sent)} {token2.symb}</span>
                                <span sx={style.swapInfo}><span>Pool Fees</span>0.3%</span>
                            </div>

                            {user.address ?
                                <button sx={swapLoading ? { ...style.swapButton, ...style.swapButtonLoading } : style.swapButton} onClick={() => swapLoading ? null : sendSwap()}>{swapLoading ? "" : "Swap"}</button>
                                :
                                <ConnectWallet2 />
                            }


                            <div sx={style.selectToken}>

                                <h2><div sx={style.close} onClick={resetSelect} />Select Currency</h2>
                                <div sx={style.inputBar}>
                                    <input type="text" id="search" required={true} placeholder=" " autoComplete="off" onChange={searchChange} value={search} />
                                    <label htmlFor="search">Search for a token</label>
                                </div>

                                <div sx={style.tokensList}>

                                    {tokensList.map((token: token, index: number) => {
                                        return (
                                            <div key={"token" + index} sx={style.tokenChoice} onClick={() => selectToken(token)}>
                                                <img src={token.icon_url} />
                                                <p>{token.name}<span>{token.symb}</span></p>
                                            </div>
                                        )
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

export default Swap;