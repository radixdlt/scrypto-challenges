// SPDX-License-Identifier: MIT

pragma solidity ^0.8.7;

import "./interfaces/MarketInterface.sol";
import "./interfaces/MLTTokenInterface.sol";
import "./interfaces/MTokenInterface.sol";
import "./PriceConsumerV3.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/access/Ownable.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/IERC20.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/math/SafeMath.sol";

contract Market is MarketInterface, Ownable {
    
  IERC20 dai;
  MTokenInterface mDai;   // mToken for DAI deposits
  MTokenInterface mMatic; // mToken for collateral (MATIC) deposits
  MLTTokenInterface MLT;  // Platform Token

  // Chainlink Oracle
  PriceConsumerV3 private oracle;
  // using SafeMath for uint256;

  // 80%, so on collateral 100x user can borrow 80x
  uint256 public maticCollateralFactor = 80;  

  uint256 public marketSizeDai = 0;    // Total deposits in DAI (*1e18)
  uint256 public totalBorrowedDai = 0; // Total Borrowed (*1e18)
  uint256 public depositAPRDai = 12;    // (*100)  12% APR Initially
  uint256 public borrowAPRDai = 14;     // (*100) [Variable borrow APR] 14% APR Initially
  
  uint256 exchangeRateDai = 1e16;      // (*1e18) 1 mDai = 0.01 Dai [Initially]
  uint256 deploymentTime;

  mapping(address => Vault) vaults;

  constructor(address _dai, address _mDai, address _mMatic, address _MLT, address _oracle) {
    dai = IERC20(_dai);
    mDai = MTokenInterface(_mDai);
    mMatic = MTokenInterface(_mMatic);
    MLT = MLTTokenInterface(_MLT);
    oracle = PriceConsumerV3(_oracle);
    deploymentTime = block.timestamp;
  }

  // false => deposit, true => withdraw
  modifier updateInterestRates(uint256 _amount, bool _isWithdrawn) {
    if(_isWithdrawn) {
      uint256 weightedVariableRate = (_amount * borrowAPRDai)/100; // (*100)
      uint256 newBorrowRate = ((weightedVariableRate * 100) / (totalBorrowedDai + _amount)); // (*100)
      
      if(marketSizeDai > 0) {
        depositAPRDai = (newBorrowRate*(totalBorrowedDai+_amount))/marketSizeDai;
      }
    
      borrowAPRDai = newBorrowRate;
    }
    else {
      uint256 weightedVariableRate = (_amount * depositAPRDai)/100; // (*100)
      uint256 newDepositRate = ((weightedVariableRate * 100) / (marketSizeDai + _amount)); // (*100)
      
      if(totalBorrowedDai > 0) {
        borrowAPRDai = (newDepositRate*(marketSizeDai+_amount))/totalBorrowedDai;
      }
      
      depositAPRDai = newDepositRate;
    }
    _;
  }

  function getExchangeRateDai() public view returns (uint256) {
    // Increasing exchange rate of mDai (12% for a year)
    // return exchangeRateDai + 
    // uint256((12 * (block.timestamp - deploymentTime)) / (3.154 * 1e7));
    return uint256(exchangeRateDai + 1e9*(block.timestamp - deploymentTime)); // For demonstartion
  }

  // Deposit DAI to earn interest (Lending)
  function deposit(uint256 _depositAmount) override external updateInterestRates(_depositAmount, false) {
    require(dai.balanceOf(msg.sender) >= _depositAmount, "Insuffiecient Balance");

    dai.transferFrom(msg.sender, address(this), _depositAmount);
    // mToken for DAI whose exchange rate against DAI will increase gradually
    uint256 amountMinted = (_depositAmount*1e18)/getExchangeRateDai();
    mDai.mint(msg.sender, amountMinted);
    // Incentive in the form of MULTI Token (100%)
    MLT.mint(msg.sender, _depositAmount);

    marketSizeDai += _depositAmount;
    vaults[msg.sender].depositAmount += _depositAmount;

    emit Deposit(_depositAmount, amountMinted);
  }

  // Withdraw DAI (deposited+interest)
  function withdraw(uint256 _withdrawalAmount) override external updateInterestRates(_withdrawalAmount, true) {
    require(_withdrawalAmount <= ((mDai.balanceOfToken(msg.sender)*getExchangeRateDai())/1e18), "Withdrawal limit exceeded");
    
    dai.transfer(msg.sender, _withdrawalAmount);
    uint256 amountBurned = (_withdrawalAmount*1e18)/getExchangeRateDai();
    mDai.burn(msg.sender, amountBurned);

    marketSizeDai -= _withdrawalAmount;
    vaults[msg.sender].withdrawnAmount += _withdrawalAmount;

    emit Withdraw(_withdrawalAmount, amountBurned);
  }

  // Deposit MATIC collateral in exchange for DAI
  function borrow(uint256 _collateralAmount, uint256 _borrowAmount) override payable external updateInterestRates(_borrowAmount, true) {
    require(_borrowAmount <= (marketSizeDai-totalBorrowedDai), "Not enough liquidity");
    require(_collateralAmount == msg.value, "Incorrect MATIC amount sent");
    
    dai.transfer(msg.sender, _borrowAmount);
    mMatic.mint(msg.sender, _collateralAmount);
    // Incentive in the form of MULTI Token (100%)
    MLT.mint(msg.sender, _borrowAmount);

    vaults[msg.sender].collateralAmount += _collateralAmount;
    vaults[msg.sender].debtAmount += _borrowAmount;
    uint256 repaymentAmount = (_borrowAmount*1e18)/getExchangeRateDai();
    vaults[msg.sender].repayAmount += repaymentAmount;
    totalBorrowedDai += _borrowAmount;
  
    emit Borrow(_collateralAmount, _borrowAmount);
  }
    
  // Allows a user to withdraw up to 100% of the collateral they have on deposit
  function repay(uint256 _repaymentAmount) override external updateInterestRates(_repaymentAmount, false) {
    require(dai.balanceOf(msg.sender) >= _repaymentAmount, "Not enough token balance");

    dai.transferFrom(msg.sender, address(this), _repaymentAmount);
    uint256 amountToWithdraw = estimateCollateralAmount(_repaymentAmount, msg.sender);
    mMatic.burn(msg.sender, _repaymentAmount);
    
    vaults[msg.sender].collateralAmount -= amountToWithdraw;
    vaults[msg.sender].repaidAmount += _repaymentAmount;
    vaults[msg.sender].repayAmount -= ((_repaymentAmount * 1e18) / getExchangeRateDai());
    totalBorrowedDai -= _repaymentAmount;

    payable(msg.sender).transfer(amountToWithdraw);
    
    emit Repay(amountToWithdraw, _repaymentAmount);
  }

  // Get Vault details   
  function getVault(address _userAddress) external view returns (Vault memory vault) {
    return vaults[_userAddress];
  }
    
  // How much MATIC will be reedemed for the given repay of DAI (with interest)
  function estimateCollateralAmount(uint256 _repaymentAmount, address account) public view returns (uint256 collateralAmount) {
    if(vaults[account].repayAmount <= 0) {
      return vaults[account].repayAmount;    
    }
    else {
      return uint256(
        (_repaymentAmount * 1e18 * vaults[account].collateralAmount) 
        / 
        (vaults[account].repayAmount * getExchangeRateDai())
      );
    }
  }
    
  // How much DAI can be borrowed for a given amount of MATIC collateral
  function estimateTokenAmount(uint256 _depositAmount) public view returns (uint256 tokenAmount) {
    return uint256(
      (_depositAmount * maticCollateralFactor * getMATICUSDPrice()) 
      / 
      (100 * getDAIUSDPrice())
    );
  }

  // How much MATIC collateral needs to be deposited for a specified DAI amount
  function estimateCollateralAmountTobePaid(uint256 _requestAmount) public view returns (uint256 collateralAmount) {
    return uint256(
      (_requestAmount * 100 * getDAIUSDPrice()) 
      / 
      (maticCollateralFactor * getMATICUSDPrice())
    );
  }

  // How much DAI can be withdrawn for a given mDai amount
  function estimateWithdrawAmount(uint256 _withdrawalAmount) public view returns (uint256 tokenAmount) {
    return uint256((_withdrawalAmount * getExchangeRateDai()) / 1e18);
  }

  function getMATICUSDPrice() public view returns (uint256){
    uint price8 = uint(oracle.getLatestPriceMatic());
    return price8*(10**10);
  }

  function getDAIUSDPrice() public view returns (uint256){
    uint price8 = uint(oracle.getLatestPriceDai());
    return price8*(10**10);
  }

  function getDaiTokenAddress() public view returns (address){
    return address(dai);
  }

  function setOracle(address _oracle) public onlyOwner {
    oracle = PriceConsumerV3(_oracle);
  }

  function getOracle() public view returns (address) {
    return address(oracle);
  }

  function setMaticCF(uint256 _collateralFactor) public onlyOwner {
    maticCollateralFactor = _collateralFactor;
  }
}