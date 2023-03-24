import {
  getTokensInWallet,
  getFunds,
  getFundPrice,
  getFundName,
  getFundImage,
} from "./apiDataFetcher";

//NB! call updatefunctions in apiDataFecther before you use these

//get all sharetokens that you have in wallet
// export function getSharetokensWallet() {
//   const funds = getFunds();
//   const tokenBalances = getTokensInWallet();
//   const matchingTokens = [];

//   for (const [tokenAddress, tokenAmount] of tokenBalances) {
//     if (funds[2].includes(tokenAddress)) {
//       const fundAddress = funds[funds[2].indexOf(tokenAddress)][0];
//       matchingTokens.push([tokenAddress, tokenAmount, fundAddress]);
//     }
//   }

//   return matchingTokens;
// }

export function getSharetokensWallet() {
  const funds = getFunds();
  const tokenBalances = getTokensInWallet();
  const matchingTokens = [];

  for (const [tokenAddress, tokenAmount] of tokenBalances) {
    const matchingFund = funds.find((fund) => fund[2] === tokenAddress);
    if (matchingFund) {
      matchingTokens.push([tokenAddress, tokenAmount, matchingFund[0]]);
    }
  }

  return matchingTokens;
}

//get all info for the my investments page, except history of the fund prices.
export function getPortfolio() {
  const myfunds = getSharetokensWallet();
  const portfolio = new Map();
  let totalUsdValue = 0;

  for (const fund of myfunds) {
    const shareTokenAddress = fund[0];
    const amount = fund[1];
    const fundAddress = fund[2];
    const fundName = getFundName(fundAddress);
    const imageLink = getFundImage(fundAddress);
    const price = getFundPrice(fundAddress);
    const usdValue = price * amount;
    const percentage = 0; //updates later

    portfolio.set(fundAddress, {
      shareTokenAddress,
      fundName,
      imageLink,
      amount,
      usdValue,
      percentage,
    });

    totalUsdValue += usdValue;
  }

  for (const [fundAddress, data] of portfolio) {
    const percentage = (data.usdValue / totalUsdValue) * 100;
    data.percentage = percentage;
    portfolio.set(fundAddress, data);
  }

  return [portfolio, totalUsdValue];
}
