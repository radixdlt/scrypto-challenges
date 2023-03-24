import {
  tokensInfo,
  getFundAmounts,
  getFundTvl,
  getFunds,
  getTokensInWallet,
  getShareTokenAddress,
  getShareTokenAmount,
  getTokenPrice,
  getFundName,
  getFundImage,
} from "./apiDataFetcher";

//NB! call updatefunctions in apiDataFecther before you use these

//returns the funds that logged in user (if you have updatetd tokens in wallet) are fundmanagers for.
export function getFundManagerFunds() {
  const funds = getFunds();
  const tokens = getTokensInWallet();
  const matchingFunds = funds.filter((fund) => tokens.has(fund[1]));
  const fundInfo = new Map();
  for (const fund of matchingFunds) {
    const fundAddr = fund[0];
    const fundManagerBage = fund[1];
    fundInfo.set(fundAddr, [
      getFundName(fundAddr),
      getFundImage(fundAddr),
      fundManagerBage,
    ]);
  }
  return fundInfo;
}

export function getYourShareAndTvl(fundAddress) {
  const shareTokenAddress = getShareTokenAddress(fundAddress);
  const amount = getShareTokenAmount(fundAddress);
  const tokenBalances = getTokensInWallet();
  const tvl = getFundTvl(fundAddress);
  const yourAmount = tokenBalances.get(shareTokenAddress) || 0;
  const yourShare = (yourAmount * tvl) / amount;
  return [yourShare, tvl];
}

export function getManageFundPortfolio(fundAddress) {
  const fundAmounts = getFundAmounts(fundAddress);
  let totalUsdValue = 0;

  const portfolio = new Map();
  for (const [tokenAddress, amount] of fundAmounts) {
    const usdValue = getTokenPrice(tokenAddress) * amount;
    const { name, ticker, image } = tokensInfo.get(tokenAddress);

    totalUsdValue += usdValue;

    portfolio.set(tokenAddress, {
      name,
      ticker,
      image,
      amount,
      usdValue,
    });
  }

  for (const [tokenAddress, data] of portfolio) {
    const percentage = (data.usdValue / totalUsdValue) * 100;
    data.percentage = percentage;
    portfolio.set(tokenAddress, data);
  }

  return portfolio;
}
