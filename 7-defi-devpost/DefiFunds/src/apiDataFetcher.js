import axios from "axios";
import { DefiFundsComponentAddress } from "./index.js";

export const xrdAddr =
  "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp";

export const tokensInfo = new Map([
  [
    "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp",
    { name: "Radix", ticker: "XRD", image: "https://example.com/token1.png" },
  ],
  [
    "resource_tdx_b_1qpev6f8v2su68ak5p2fswd6gqml3u7q0lkrtfx99c4ts3zxlah",
    {
      name: "Beta Usd",
      ticker: "BUSD",
      image: "https://example.com/token2.png",
    },
  ],
  [
    "resource_tdx_b_1qps68awewmwmz0az7cxd86l7xhq6v3pez355wq8gra3qw2v7kp",
    {
      name: "Wrapped Ether",
      ticker: "WETH",
      image: "https://example.com/token3.png",
    },
  ],
  [
    "resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r",
    {
      name: "Wrapped Bitcoin",
      ticker: "WBTC",
      image: "https://example.com/token2.png",
    },
  ],
]);

// all funds in defifunds array each element consists off (fundAddr, shartokenAddr, fundManagerBage)
let funds = [];

// info on funds you have put into here. how updated the data is depends on when you last updated the fund info on that speciific fund. map (fundAddress, data_json)
let fundsInfo = new Map();

//All fungible tokens in your wallet
let tokensInWallet = new Map();

//tokenPrices in usd. Prices on those token addresses defined in addr
let tokenPrices = new Map();

///////////////////////////////////////
////////////// API Calls //////////////
///////////////////////////////////////

//NB! you will it most cases call the updatefunctions, and the getfunctions not these fetch functions.

export async function fetchPoolInfo(tokenX, tokenY) {
  const apiUrl = "https://beaker.fi:8888/pool_info_price?";
  const params = `token_x=${tokenX}&token_y=${tokenY}`;

  try {
    const response = await axios.get(apiUrl + params);
    const poolInfo = response.data;
    return poolInfo;
  } catch (error) {
    console.error(error);
  }
}

export async function fetchRadixPrice() {
  return axios
    .get(
      "https://api.coingecko.com/api/v3/simple/price?ids=radix&vs_currencies=usd"
    )
    .then((response) => {
      const price = response.data.radix.usd;
      return price;
    })
    .catch((error) => {
      console.error(error);
    });
}

export async function fetchFundInfo(fundAddress) {
  return axios
    .post("https://betanet.radixdlt.com/entity/details", {
      address: fundAddress,
    })
    .then((response) => {
      return response.data.details.state.data_json;
    });
}

export async function fetchFunds() {
  return axios
    .post("https://betanet.radixdlt.com/entity/details", {
      address: DefiFundsComponentAddress,
    })
    .then((response) => {
      return response.data.details.state.data_json[0];
    });
}

export async function fetchFungibleTokens(address) {
  return axios
    .post("https://betanet.radixdlt.com/entity/fungibles", {
      address: address,
    })
    .then((response) => {
      let vector = response.data.fungibles.items;
      const tokenBalances = new Map();
      for (const item of vector) {
        const tokenAddress = item.address;
        const tokenAmount = parseFloat(item.amount.value);
        tokenBalances.set(tokenAddress, tokenAmount);
      }
      return tokenBalances;
    });
}

export async function fetchAllTokenPricesXrd() {
  // fetch all prices in paralell
  const promises = [];
  tokensInfo.forEach((tokenInfo, tokenAddress) => {
    if (tokenAddress !== xrdAddr) {
      const promise = fetchPoolInfo(tokenAddress, xrdAddr).then((data) => ({
        tokenAddress,
        price: data[1] / data[0],
      }));
      promises.push(promise);
    }
  });
  const results = await Promise.all(promises);
  const prices = {};
  results.forEach(({ tokenAddress, price }) => {
    prices[tokenAddress] = price;
  });
  prices[xrdAddr] = 1;
  return prices;
}

/////////////////////////////////////////////////
////////////// Update global variables //////////
/////////////////////////////////////////////////

//NB! call these functions before you use the get functions

//in usd
export async function updateTokenPrices() {
  const [xrdPrice, prices] = await Promise.all([
    fetchRadixPrice(),
    fetchAllTokenPricesXrd(),
  ]);
  for (const [tokenAddress, price] of Object.entries(prices)) {
    tokenPrices.set(tokenAddress, price * xrdPrice);
  }
}

export async function updateFunds() {
  const f = await fetchFunds();
  funds = f;
}

//input: vec<fundAddr>
export async function updateFundsInfo(funds) {
  const promises = funds.map((fundAddr) => fetchFundInfo(fundAddr));
  const results = await Promise.all(promises);

  for (let i = 0; i < funds.length; i++) {
    fundsInfo.set(funds[i], results[i]);
  }
}

export async function updateTokensInWallet(address) {
  const tokens = await fetchFungibleTokens(address);
  tokensInWallet = tokens;
}

export async function updateAll(walletAddr) {
  await updateFunds();
  const funds = getFunds();
  const fundAddresses = funds.map((fund) => fund[0]);
  const promises = [
    updateFundsInfo(fundAddresses),
    updateTokenPrices(),
    updateTokensInWallet(walletAddr),
  ];
  await Promise.all(promises);
}

///////////////////////////////////
////////////// Get stuff //////////
///////////////////////////////////

//NB! you need to call update function before you will get stuff from these functions

export function getTokenPrices() {
  return tokenPrices;
}

export function getFunds() {
  return funds;
}
export function getFundsInfo() {
  return fundsInfo;
}

export function getTokensInWallet() {
  return tokensInWallet;
}

export function getTokenPrice(tokenAddress) {
  return tokenPrices.get(tokenAddress) || null;
}

export function getFundInfo(fundAddress) {
  return fundsInfo.get(fundAddress) || null;
}

export function getTokenAmount(tokenAddress) {
  const tokensInWallet = getTokensInWallet(tokenAddress);
  const tokenAmount = tokensInWallet.get(tokenAddress) ?? 0;
  return tokenAmount;
}

export function getFundName(fundAddr) {
  return getFundInfo(fundAddr)[0];
}

export function getFundStrategy(fundAddr) {
  return getFundInfo(fundAddr)[1];
}

export function getFundImage(fundAddr) {
  return getFundInfo(fundAddr)[2];
}

export function getFundWebsite(fundAddr) {
  return getFundInfo(fundAddr)[3];
}

//will likly do this another way when rc net is coming. can check resource amount in vault directly
export function getFundAmounts(fundAddr) {
  const fundAmounts = getFundInfo(fundAddr)[4];
  let map = new Map();
  for (let e of fundAmounts) {
    map.set(e[0], e[1][1]);
  }
  return map;
}

export function getFundManagerBadge(fundAddr) {
  return getFundInfo(fundAddr)[5];
}

export function getShareTokenAddress(fundAddr) {
  return getFundInfo(fundAddr)[7];
}

//will likly do this another way when rc net is coming. amount directly then
export function getShareTokenAmount(fundAddr) {
  return getFundInfo(fundAddr)[8];
}

export function getDepositFee(fundAddr) {
  return getFundInfo(fundAddr)[10];
}

export function getFundTokenAmount(fundAddr, tokenAddr) {
  const fundAmounts = getFundAmounts(fundAddr);
  const tokenAmount = fundAmounts.get(tokenAddr) ?? 0;
  return tokenAmount;
}

export function getFundTvl(FundAddress) {
  const fundAmounts = getFundAmounts(FundAddress);
  let totalValue = 0;

  for (let [tokenAddress, amount] of fundAmounts) {
    const price = getTokenPrice(tokenAddress);
    const value = price * amount;
    totalValue += value;
  }
  return totalValue;
}

export function getFundPrice(FundAddress) {
  const tvl = getFundTvl(FundAddress);
  const amount = getShareTokenAmount(FundAddress);
  return tvl / amount;
}

export function getAllFundAmounts(funds) {
  const amounts = funds.map((fund) => {
    const fundAmounts = getFundAmounts(fund);
    return [fund, fundAmounts];
  });

  return new Map(amounts);
}

export function getAllShareTokenAmounts(funds) {
  const amounts = funds.map((fund) => {
    const shareTokenAmount = getShareTokenAmount(fund);
    return [fund, shareTokenAmount];
  });

  return new Map(amounts);
}

export function getAllFundTvls(funds) {
  const tvls = funds.map((fund) => {
    const fundTvl = getFundTvl(fund);
    return [fund, fundTvl];
  });

  return new Map(tvls);
}

export function getAllFundPrices(funds) {
  const prices = funds.map((fund) => {
    const fundPrice = getFundPrice(fund);
    return [fund, fundPrice];
  });

  return new Map(prices);
}

export async function getRatios(FundAddress) {
  await Promise.all([updateTokenPrices(), updateFundsInfo([FundAddress])]); //important to have as new as possible here, since you use it for smart contratc integration. no dangers, if not, but will get bigger rest amounts.
  const amounts = getFundAmounts(FundAddress);
  let totalValue = 0;
  let values = new Map();

  for (let [address, amount] of amounts.entries()) {
    let value = amount * getTokenPrice(address);
    values.set(address, value);
    totalValue += value;
  }
  let ratios = new Map();
  for (let [address, value] of values.entries()) {
    let ratio = value / totalValue;
    ratios.set(address, ratio);
  }
  return ratios;
}
