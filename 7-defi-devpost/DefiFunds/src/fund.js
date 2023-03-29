import {
  getFundTvl,
  getShareTokenAddress,
  getShareTokenAmount,
  getTokensInWallet,
  getFundAmounts,
  getTokenPrice,
  tokensInfo,
} from "./apiDataFetcher";

//NB! call updatefunctions in apiDataFecther before you use these

export function getYourShareAndTvl(fundAddress) {
  const sharetokenAddress = getShareTokenAddress(fundAddress);
  const amount = getShareTokenAmount(fundAddress);
  const tokenBalances = getTokensInWallet();
  const tvl = getFundTvl(fundAddress);
  const yourAmount = tokenBalances.get(sharetokenAddress) || 0;
  const yourShare = (yourAmount * tvl) / amount;
  return [yourShare, tvl];
}

export function getFundPortfolio(fundAddress) {
  const fundAmounts = getFundAmounts(fundAddress);
  let totalUsdValue = 0;

  const portfolio = new Map();
  for (const [tokenAddress, amount] of fundAmounts) {
    const usdValue = getTokenPrice(tokenAddress) * amount;
    totalUsdValue += usdValue;
    portfolio.set(tokenAddress, usdValue);
  }

  for (const [tokenAddress, usdValue] of portfolio) {
    const percentage = (usdValue / totalUsdValue) * 100;
    const { name, ticker, image } = tokensInfo.get(tokenAddress);
    portfolio.set(tokenAddress, { name, ticker, image, percentage });
  }

  return portfolio;
}
