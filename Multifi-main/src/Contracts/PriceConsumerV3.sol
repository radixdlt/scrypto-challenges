// SPDX-License-Identifier: MIT

pragma solidity ^0.8.7;

import "./interfaces/AggregatorV3Interface.sol";

contract PriceConsumerV3 {
  AggregatorV3Interface internal priceFeedMatic;
  AggregatorV3Interface internal priceFeedDai;

  constructor() {
    // For Polygon Mumbai Testnet
    priceFeedMatic = AggregatorV3Interface(0xd0D5e3DB44DE05E9F294BB0a3bEEaF030DE24Ada);
    priceFeedDai = AggregatorV3Interface(0x0FCAa9c899EC5A91eBc3D5Dd869De833b06fB046);
  }

  function getLatestPriceMatic() public view returns (int) {
    (uint80 roundID, int price, uint startedAt, uint timeStamp, uint80 answeredInRound) = priceFeedMatic.latestRoundData();

    return price;
  }

  function getLatestPriceDai() public view returns (int) {
    (uint80 roundID, int price, uint startedAt, uint timeStamp, uint80 answeredInRound) = priceFeedDai.latestRoundData();
    
    return price;
  }
}