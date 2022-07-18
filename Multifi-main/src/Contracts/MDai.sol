// SPDX-License-Identifier: MIT

pragma solidity ^0.8.7;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol";
import "./interfaces/MTokenInterface.sol";

contract MDai is MTokenInterface, ERC20 {
    address public owner;
    address public allowedContract; // Market
    
    uint public rewardRate = 1e14; // Just for demonstration
    uint public lastUpdateTime;
    uint public rewardPerTokenStored;

    mapping(address => uint) public userRewardPerTokenPaid;
    mapping(address => uint) public rewards;

    uint private _totalSupply;
    mapping(address => uint) public _balances;

    constructor(string memory _name, string memory _symbol) ERC20(_name, _symbol) {
        owner = msg.sender;
        allowedContract = msg.sender;
    }

    modifier onlyOwner() {
      require(msg.sender == owner || msg.sender == allowedContract);
      _;
    }

    function setAllowedContract(address _contractAddress) onlyOwner public {
      allowedContract = _contractAddress;
    }

    function rewardPerToken() public view returns (uint) {
        if (_totalSupply == 0) {
            return 0;
        }
        return
            rewardPerTokenStored +
            (((block.timestamp - lastUpdateTime) * rewardRate * 1e18) / _totalSupply);
    }

    function earned(address account) public view returns (uint) {
        return
            ((_balances[account] *
                (rewardPerToken() - userRewardPerTokenPaid[account])) / 1e18) +
            rewards[account];
    }

    modifier updateReward(address account) {
        rewardPerTokenStored = rewardPerToken();
        lastUpdateTime = block.timestamp;

        rewards[account] = earned(account);
        userRewardPerTokenPaid[account] = rewardPerTokenStored;
        _;
    }

    function stake(uint _amount) external payable updateReward(msg.sender) {
        _totalSupply += _amount;
        _balances[msg.sender] += _amount;
        _burn(msg.sender,  _amount);
    }

    function withdraw(uint _amount) external updateReward(msg.sender) {
        _totalSupply -= _amount;
        _balances[msg.sender] -= _amount;
        _mint(msg.sender, _amount);
    }

    function getReward() external updateReward(msg.sender) {
        uint reward = rewards[msg.sender];
        rewards[msg.sender] = 0;
        _mint(msg.sender, reward);
    }

    function mint(address account, uint256 amount) onlyOwner external override returns(bool){
        _mint(account, amount);
        return true;
    }

    function burn(address account, uint256 amount) onlyOwner external override returns(bool){
        _burn(account, amount);
        return true;
    }

    function balanceOfToken(address account) public view virtual override returns (uint256) {
        return balanceOf(account);
    }
}