import { getFundTvl, getFunds, getFundImage } from "./apiDataFetcher";

//NB! call updatefunctions in apiDataFecther before you use these

export async function getFundsSortedTvl() {
  const funds = getFunds();
  const tvls = [];

  for (const fund of funds) {
    const fundAddr = fund[0];
    const tvl = getFundTvl(fundAddr);
    tvls.push([fundAddr, getFundName(fundAddr), getFundImage(fundAddr), tvl]);
  }

  const sortedTvls = tvls.sort((a, b) => b[3] - a[3]);
  return sortedTvls;
}
