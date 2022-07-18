// SPDX-License-Identifier: MIT

pragma solidity ^0.8.7;

interface MarketInterface {
  struct Vault {
    uint256 depositAmount; // Amount of DAI deposited
    uint256 withdrawnAmount; // Amount of DAI withdrawn
    uint256 collateralAmount; // Amount of collateral held by the vault contract
    uint256 debtAmount; // Amount of DAI borrowed
    uint256 repaidAmount; // Amount of DAI repaid
    uint256 repayAmount; // Amount of mDAI equivalence 
  }

  // Event definitions
  event Deposit(uint256 amountDeposited, uint256 amountMinted);
  event Withdraw(uint256 collateralWithdrawn, uint256 amountBurned);
  event Borrow(uint256 collateralDeposited, uint256 amountBorrowed);
  event Repay(uint256 amoutWithdrawn, uint256 amountRepayed);
    
  // Function definitions
  function getExchangeRateDai() external view returns (uint256);

  function deposit(uint256 _depositAmount) external;
  
  function withdraw(uint256 _withdrawalAmount) external;
    
  function borrow(uint256 _collateralAmount, uint256 _borrowAmount) payable external;

  function repay(uint256 _repaymentAmount) external;

  function getVault(address _userAddress) external view returns (Vault memory vault);
    
  function estimateCollateralAmount(uint256 _repaymentAmount, address account) external view returns (uint256 collateralAmount);
    
  function estimateTokenAmount(uint256 _depositAmount) external view returns (uint256 tokenAmount);

  function estimateCollateralAmountTobePaid(uint256 _requestAmount) external view returns (uint256 collateralAmount);

  function estimateWithdrawAmount(uint256 _withdrawalAmount) external view returns(uint256 tokenAmount);

  function getMATICUSDPrice() external view returns (uint256);

  function getDAIUSDPrice() external view returns (uint256);

  function getDaiTokenAddress() external view returns (address);

  function setOracle(address _oracle) external;

  function getOracle() external view returns (address);

  function setMaticCF(uint256 _collateralFactor) external;
}