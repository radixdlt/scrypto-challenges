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

import { formatToString, formatToString2, randomIntFromInterval, twoDecimals } from "utils/general/generalMaths";

import { stable_coin as stable, token_default } from "utils/general/constants";

import { addLiquidityNoPosition, addLiquidityToPosition, claimFees, removeAllLiquidity } from "utils/dex/routerContractCalls";

import styleFunction from "./style";

import { token, position } from "types";

function Liquidity() {

    let [searchParams, setSearchParams] = useSearchParams();

    const [stars, setStars] = useState(Array.from({ length: 10 }, (_, i) => [randomIntFromInterval(0, 1), randomIntFromInterval(10, 90), randomIntFromInterval(10, 90), randomIntFromInterval(0, 1)]));

    const { addAlert } = useContext(SnackbarContext);

    const { device } = useContext(ResponsiveContext);

    const { tokens, pools } = useContext(TokensContext);

    const { user, tokensOwned, setNbTokens, positions } = useContext(UserContext);

    const [tokensList, setTokensList] = useState(tokens.filter((x: token) => x.address != stable.address));

    const [price, setPrice] = useState<number>(0);

    const [sent, setSent] = useState<number>(0);
    const [get, setGet] = useState<number>(0);

    const [minPrice, setMinPrice] = useState<number>(1);
    const [maxPrice, setMaxPrice] = useState<number>(100);
    const [price1, setPrice1] = useState<number>(1);
    const [price2, setPrice2] = useState<number>(100);

    const [myPosition, setMyPosition] = useState(false);

    const [chosePosition, setChosePosition] = useState(false);

    const [token1, setToken1] = useState(token_default);

    const [token1Select, setToken1Select] = useState(false);

    const [search, setSearch] = useState("");

    const [searchPosition, setSearchPosition] = useState("");

    const [swapLoading, setSwapLoading] = useState(false);

    const [positionsList, setPositionsList] = useState(positions);

    const [nftId, setNftId] = useState<string | null>(null);

    const [positionInfos, setPositionInfos] = useState<position>({
        token: token_default,
        liquidity: 0,
        price_x: 0,
        value_locked: '?',
        x_fees: '?',
        y_fees: '?',
        nfIdValue: null,
        id: null,
    });

    const [feesLoading, setFeesLoading] = useState(false);

    const [removeLoading, setRemoveLoading] = useState(false);



    function resetValues() {
        setSent(0);
        setGet(0);
    }

    useEffect(() => {
        var tk1 = searchParams.get('tk1');

        if (!tk1) {
            setSearchParams({ tk1: token_default.symb })
        }
    }, [])

    useEffect(() => {
        if (pools[token1.address]) {
            setMinPrice(parseFloat(pools[token1.address]["min_rate"]));
            setMaxPrice(parseFloat(pools[token1.address]["max_rate"]));
            setPrice1(parseFloat(pools[token1.address]["min_rate"]));
            setPrice2(parseFloat(pools[token1.address]["max_rate"]))
        } else {
            setMinPrice(1);
            setMaxPrice(100);
            setPrice1(1);
            setPrice2(100);
        }
    }, [token1, pools])

    useEffect(() => {
        if (tokens) {
            var tk1 = searchParams.get('tk1');

            if (tk1) {
                tk1 = tk1.toLowerCase();
            }

            if (tk1 && tokens.map((x: token) => x.symb.toLowerCase()).includes(tk1)) {
                var tok1 = tokens.filter((x: token) => x.symb.toLowerCase() == tk1)[0]
                setToken1({ name: tok1!.name, symb: tok1!.symb, address: tok1!.address, icon_url: tok1!.icon_url });
                setSearchParams({ tk1: tk1!.toUpperCase() })
            } else {
                setToken1(token_default);
                setSearchParams({ tk1: token_default.symb })
            }
        }

    }, [tokens])

    const token1AddressRef = useRef(token1.address)
    token1AddressRef.current = token1.address


    function findRatio(x: number) {
        var currentStep = pools[token1.address]["current_step"];

        let stableRatio: number;
        for (var i = 0; i < pools[token1.address]["steps"].length; ++i) {
            const step = pools[token1.address]["steps"][i];

            if (step.step_id == currentStep) {
                stableRatio = parseFloat(step.stablecoin_amount) / (parseFloat(step.stablecoin_amount) + parseFloat(step.rate) * parseFloat(step.other_token_amount));

                console.log(parseFloat(step.stablecoin_amount));
                console.log(parseFloat(step.other_token_amount));
                console.log(stableRatio);

                return [isNaN(stableRatio) ? 1 : stableRatio, parseFloat(step.rate)]
            }
        }
        return [1, 1];
    }


    function calculateGet(x: number) {
        var result = findRatio(x);
        var stableRatio = result[0];
        var price = result[1];
        var currentStep = pools[token1.address]["current_step"];
        var minStep = Math.ceil(Math.log(Math.min(price1, price2) / parseFloat(pools[token1.address]["min_rate"])) / Math.log(pools[token1.address]["rate_step"]));
        var maxStep = Math.floor(Math.log(Math.max(price1, price2) / parseFloat(pools[token1.address]["min_rate"])) / Math.log(pools[token1.address]["rate_step"]));

        if (currentStep > maxStep) { setSent(0); return 10; }
        if (currentStep < minStep) return 0;
        if (stableRatio == 0) return 0;
        if (stableRatio == 1) return x * price;
        return ((currentStep - minStep + 1) * (x / (maxStep - currentStep + 1) * price) * stableRatio / (1 - stableRatio))
    }


    function calculateSent(x: number) {
        var result = findRatio(x);
        var stableRatio = result[0];
        var price = result[1];
        var currentStep = pools[token1.address]["current_step"];
        var minStep = Math.ceil(Math.log(Math.min(price1, price2) / parseFloat(pools[token1.address]["min_rate"])) / Math.log(pools[token1.address]["rate_step"]));
        var maxStep = Math.floor(Math.log(Math.max(price1, price2) / parseFloat(pools[token1.address]["min_rate"])) / Math.log(pools[token1.address]["rate_step"]));

        if (currentStep > maxStep) { return 0; }
        if (currentStep < minStep) { setGet(0); return 10; }
        if (stableRatio == 0) return 0;
        if (stableRatio == 1) return x / price;
        return ((maxStep - currentStep + 1) * (x / (currentStep - minStep + 1) / price) * (1 - stableRatio) / (stableRatio))
    }

    const range1Change = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                setPriceMin(0);
                return
            }
            if (s.includes(".")) {
                setPriceMin(s);
            } else {
                setPriceMin(parseFloat(s));
            }
        }
    }

    const range2Change = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                setPriceMax(0);
                return
            }
            if (s.includes(".")) {
                setPriceMax(s);
            } else {
                setPriceMax(parseFloat(s));
            }
        }
    }


    useEffect(() => { if (pools[token1.address]) setGet(calculateGet(sent)) }, [price1, price2]);

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
                if (isNaN(x)) setGet(0);
                else setGet(x);
            } else {
                setSent(parseFloat(s));
                var x = calculateGet(parseFloat(s));
                if (isNaN(x)) setGet(0);
                else setGet(x);
            }
        }
    }

    const getChange = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                resetValues();
                return
            }
            if (s.includes(".")) {
                setGet(s);
                var x = calculateSent(parseFloat(s));
                if (isNaN(x)) setSent(0);
                else setSent(x);
            } else {
                setGet(parseFloat(s));
                var x = calculateSent(parseFloat(s));
                if (isNaN(x)) setSent(0);
                else setSent(x);
            }
        }
    }

    useEffect(() => {
        async function getPoolInfos() {
            setPrice(0);
            if (token1.address && stable.address && pools[token1.address]) {
                setPrice((parseFloat(pools[token1.address]["min_rate"]) * (parseFloat(pools[token1.address]["rate_step"]) ** parseFloat(pools[token1.address]["current_step"]))));
            }
        }
        getPoolInfos();
    }, [token1, tokensOwned])

    useEffect(() => {
        resetValues();
    }, [token1])

    function resetSelect() {
        setSearch('');
        setToken1Select(false);
    }

    function selectToken(token: token) {
        if (token1Select) {
            setToken1(token)
            setSearchParams({ tk1: token.symb.toUpperCase() })
        }
        resetSelect();
        resetValues();
    }

    function getSearch(list: token[]) {
        return list.filter(x => {
            if (x.address == stable.address) return false;
            var flag = (search.length == 0);
            for (const word of search.split(' ')) {
                if (word.length > 0) flag = flag || x.name.toLowerCase().includes(word) || x.symb.toLowerCase().includes(word)
            }
            return flag
        })
    }

    function getSearchPosition(list: position[]) {
        if (!list) return []
        return list.filter(x => {
            var flag = (searchPosition.length == 0);
            for (const word of searchPosition.split(' ')) {
                if (word.length > 0) flag = flag || x.token!.name.toLowerCase().includes(word) || x.token!.symb.toLowerCase().includes(word) || stable.name.toLowerCase().includes(word) || stable.symb.toLowerCase().includes(word)
            }
            return flag
        })
    }

    const searchChange = (event: any) => {
        setSearch(event.target.value.toLowerCase());
    }

    const searchPositionChange = (event: any) => {
        setSearchPosition(event.target.value.toLowerCase());
    }

    useEffect(() => {
        setTokensList(getSearch(tokens));
    }, [tokens, search])

    useEffect(() => {
        setPositionsList(getSearchPosition(positions));
    }, [positions, searchPosition])

    useEffect(() => {
        if (nftId == null) {
            setPositionInfos({
                token: token_default,
                liquidity: 0,
                price_x: 0,
                value_locked: '?',
                x_fees: '?',
                y_fees: '?',
                nfIdValue: null,
                id: null,
            })
        }
    }, [nftId]);

    function findPosition(tk1Address: string, tk2Address: string) {
        if (!positions) return
        for (const position of positions) {
            if (position.token.address == tk1Address) { return position.id; }
        }
        return null;
    }


    useEffect(() => {
        setNftId(findPosition(token1.address, stable.address));
    }, [token1, positions])


    async function sendSwap() {
        setSwapLoading(true);
        var flag;
        var steps: number[][] = [];
        var currentStep = parseFloat(pools[token1.address]["current_step"]);
        var minStep = Math.ceil(Math.log(Math.min(price1, price2) / parseFloat(pools[token1.address]["min_rate"])) / Math.log(pools[token1.address]["rate_step"]));
        var maxStep = Math.floor(Math.log(Math.max(price1, price2) / parseFloat(pools[token1.address]["min_rate"])) / Math.log(pools[token1.address]["rate_step"]));

        console.log(minStep, maxStep);

        for (var i = minStep; i < Math.min(currentStep, maxStep); ++i) { steps.push([i, get / (Math.min(currentStep, maxStep) - minStep + 1), 0]); }
        if (maxStep >= currentStep && minStep <= currentStep) steps.push([i, get / (Math.min(currentStep, maxStep) - minStep + 1), sent / (maxStep - Math.max(currentStep, minStep) + 1)])
        for (var i = Math.max(currentStep + 1, minStep); i <= maxStep; ++i) { steps.push([i, 0, sent / (maxStep - Math.max(currentStep, minStep) + 1)]) }

        if (!nftId) flag = await addLiquidityNoPosition(user.address, token1.address, get, sent, steps)
        else flag = await addLiquidityToPosition(user.address, token1.address, get, sent, steps, nftId)
        setNbTokens();
        resetValues();
        if (flag) {
            addAlert("check", "You have provided liquidity!");
        } else {
            addAlert("error", "An error occured");
        }
        setSwapLoading(false);
    }


    async function claimF() {
        setFeesLoading(true);
        var flag;
        if (nftId) {
            flag = await claimFees(user.address, nftId);
            if (flag) {
                addAlert("check", "Your fees have been claimed!");
            } else {
                addAlert("error", "An error occured");
            }
            setFeesLoading(false);
        }
        else { addAlert("error", "You don't have a position"); return false; }
    }


    async function removeL(liqu: string) {
        setRemoveLoading(true);
        var flag;
        if (nftId) {
            flag = await removeAllLiquidity(user.address, nftId);
            if (flag) {
                addAlert("check", "Your liquidity has been removed!");
            } else {
                addAlert("error", "An error occured");
            }
            setRemoveLoading(false);
        }
        else { addAlert("error", "You don't have a position"); return false; }
    }

    function setPriceMin(x: number) {
        if (isNaN(x)) {
            if (price1 <= price2) { setPrice1(minPrice); }
            else setPrice2(minPrice);
            return
        }
        if (x > Math.max(price1, price2)) { setPrice1(Math.max(Math.min(x, maxPrice), minPrice)); setPrice2(Math.max(Math.min(x, maxPrice), minPrice)); return }
        if (price1 <= price2) { setPrice1(Math.max(Math.min(x, maxPrice), minPrice)); }
        else setPrice2(Math.max(Math.min(x, maxPrice), minPrice));
    }

    function setPriceMax(x: number) {
        if (isNaN(x)) {
            if (price1 > price2) { setPrice1(minPrice); }
            else setPrice2(minPrice);
            return
        }
        if (x < Math.min(price1, price2)) { setPrice1(Math.max(Math.min(x, maxPrice), minPrice)); setPrice2(Math.max(Math.min(x, maxPrice), minPrice)); return }
        if (price1 > price2) setPrice1(Math.max(Math.min(x, maxPrice), minPrice));
        else setPrice2(Math.max(Math.min(x, maxPrice), minPrice));
    }




    const style = styleFunction(device, swapLoading, token1Select, chosePosition, price1, price2, minPrice, maxPrice);


    return (
        <Dashboard page="liquidity">
            <Snackbar />

            {stars.map((x, index) => {
                return (
                    <Star key={"star" + index} left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"} />
                )
            })}

            <div sx={style.main}>

                <div sx={style.top}>

                    <div sx={style.container}>

                        <div sx={style.buttons}>
                            <span sx={myPosition && user.address ? style.inactive : style.active} onClick={() => { setMyPosition(false); setChosePosition(false); }}>Provide Liquidity</span>
                            {user.address ?
                                <span sx={myPosition ? style.active : style.inactive} onClick={() => { setMyPosition(true); resetSelect(); }}>My Position</span>
                                :
                                null
                            }
                        </div>

                        {myPosition && user.address ?

                            <div sx={style.myPositionColumn}>

                                <div sx={style.chosePositionContainer}>

                                    <div sx={style.chosePositionZone}>

                                        <h2><div sx={style.close} onClick={() => setChosePosition(false)} />Your positions</h2>
                                        <div sx={style.inputBar}>
                                            <input type="text" id="searchPosition" required={true} placeholder=" " autoComplete="off" onChange={searchPositionChange} />
                                            <label htmlFor="searchPosition">Search for a position</label>
                                        </div>

                                        <div sx={style.poolsList}>
                                            {positionsList.map((position: position, index: number) => {
                                                return (
                                                    <div key={"position" + index} sx={style.poolChoice} onClick={() => {
                                                        setChosePosition(false);
                                                        setToken1(position.token!);
                                                        setNftId(position.id);
                                                        setPositionInfos(position);
                                                    }}>
                                                        <img src={position.token!.icon_url} />
                                                        <img src={stable.icon_url} />
                                                        <p>{position.token!.symb} - {stable.symb}</p>
                                                    </div>
                                                )
                                            })}
                                        </div>
                                    </div>

                                </div>

                                <div sx={style.chosePosition} onClick={() => setChosePosition(true)}>
                                    <img src={token1.icon_url} />
                                    <img src={stable.icon_url} />
                                    <p>{token1.symb} - {stable.symb}</p>
                                    <div sx={style.expand2} />
                                </div>

                                <div sx={style.swapZone}>
                                    <h1>🌻 My Fees</h1>
                                    <div sx={style.swapInfos}>
                                        <span sx={style.swapInfoMain}><span>Total Locked</span><div>? {token1.symb} + ? {stable.symb}</div></span>
                                        <span sx={style.swapInfo}><span>Value</span>${positionInfos.value_locked == "?" ? "?" : formatToString(positionInfos.value_locked)}</span>
                                        <span sx={style.swapInfo}><span>Fees</span>{positionInfos.x_fees == "?" ? "?" : formatToString2(positionInfos.x_fees)} {token1.symb} + {positionInfos.y_fees == "?" ? "?" : formatToString2(positionInfos.y_fees)} {stable.symb}</span>
                                    </div>
                                    <button sx={feesLoading ? { ...style.swapButton, ...style.swapButtonLoading } : style.swapButton} onClick={() => feesLoading ? null : claimF()}>{feesLoading ? "" : "Claim Fees"}</button>
                                </div>

                                <div sx={style.swapZone}>
                                    <h1>🍂 Remove Liquidity</h1>
                                    <div sx={style.swapInfos}>
                                        <span sx={style.swapInfoMain}><span>Removing</span><div>? {token1.symb} + ? {stable.symb}</div></span>
                                        <span sx={style.swapInfo}><span>Value</span>${positionInfos.value_locked == "?" ? "?" : formatToString(positionInfos.value_locked)}</span>
                                    </div>
                                    <button sx={removeLoading ? { ...style.swapButton, ...style.swapButtonLoading } : style.swapButton} onClick={() => removeLoading ? null : removeL('1')}>{removeLoading ? "" : "Remove Liquidity"}</button>
                                </div>

                            </div>

                            :
                            <div sx={style.swapZone}>

                                <h1>🌱 Provide Liquidity</h1>
                                <div sx={style.inputBar}>
                                    <input type="text" id="send" required={true} placeholder=" " autoComplete="off" onChange={sentChange} value={sent} />
                                    <label htmlFor="send">You lock</label>
                                    <div sx={style.token} onClick={() => setToken1Select(true)}>
                                        <img src={token1.icon_url} />
                                        <p>{token1.symb}</p>
                                        <div sx={style.expand} />
                                    </div>
                                </div>

                                <span sx={style.tokenAddress}><span>Token Address</span>{token1.address.slice(0, 5) + "..." + token1.address.slice(token1.address.length - 10, token1.address.length)}</span>

                                <div sx={style.inputBar}>
                                    <input type="text" id="get" required={true} placeholder=" " autoComplete="off" onChange={getChange} value={get} />
                                    <label htmlFor="get">You lock</label>
                                    <div sx={style.token2}>
                                        <img src={stable.icon_url} />
                                        <p>{stable.symb}</p>
                                    </div>
                                </div>

                                <span sx={style.tokenAddress}><span>Token Address</span>{stable.address.slice(0, 5) + "..." + stable.address.slice(stable.address.length - 10, stable.address.length)}</span>

                                <div sx={style.rangeInput}>
                                    <p>Price Range ({stable.symb + "/" + token1.symb})</p>
                                    <div sx={style.ranges}>
                                        <div sx={style.rangeBar}>
                                            <div />
                                        </div>
                                        <input type="range" sx={style.range2} min={0} max={1} value={Math.sqrt((price1 - minPrice) / (maxPrice - minPrice))} step={1 / 100} onChange={(e: any) => { setPrice1(twoDecimals(minPrice + parseFloat(e.target.value) ** 2 * (maxPrice - minPrice))) }} />
                                        <input type="range" sx={style.range2} min={0} max={1} value={Math.sqrt((price2 - minPrice) / (maxPrice - minPrice))} step={1 / 100} onChange={(e) => { setPrice2(twoDecimals(minPrice + parseFloat(e.target.value) ** 2 * (maxPrice - minPrice))) }} />
                                        <input type="range" sx={style.range2} min="0" max="1000" step="10" />
                                    </div>
                                    <div sx={style.rangeInputs}>
                                        <div sx={style.inputBar2}>
                                            <input type="text" id="range1" required={true} placeholder=" " autoComplete="off" onChange={(e) => { range1Change(e); }} value={Math.min(price1, price2)} />
                                            <label htmlFor="range1">Price min</label>
                                        </div>
                                        <div sx={style.inputBar2}>
                                            <input type="text" id="range2" required={true} placeholder=" " autoComplete="off" onChange={(e) => { range2Change(e); }} value={Math.max(price1, price2)} />
                                            <label htmlFor="range2">Price max</label>
                                        </div>
                                    </div>
                                </div>

                                <div sx={style.swapInfos}>
                                    <span sx={style.swapInfoMain}><span>Providing</span><div>{typeof (sent) == "string" ? formatToString(parseFloat(sent)) : formatToString(sent)} {token1.symb} + {typeof (get) == "string" ? formatToString(parseFloat(get)) : formatToString(get)} {stable.symb}</div></span>
                                    <span sx={style.swapInfo}><span>Current Price</span>1 {token1.symb} = {price == 0 ? "?" : formatToString(price)} {stable.symb}</span>
                                </div>

                                {user.address ?
                                    <button sx={swapLoading ? { ...style.swapButton, ...style.swapButtonLoading } : style.swapButton} onClick={() => swapLoading ? null : sendSwap()}>{swapLoading ? "" : "Provide Liquidity"}</button>
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

                        }
                    </div>

                </div>

            </div>

        </Dashboard>
    )
}

export default Liquidity;