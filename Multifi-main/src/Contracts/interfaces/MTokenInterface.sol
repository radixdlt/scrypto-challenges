// SPDX-License-Identifier: MIT

pragma solidity ^0.8.7;

// Interface for the MTokens
interface MTokenInterface {
    function mint(address account, uint256 amount) external returns (bool);

    function burn(address account, uint256 amount) external returns (bool);

    function rewardPerToken() external view returns (uint256);

    function earned(address account) external view returns (uint256);

    function stake(uint256 _amount) external payable;

    function withdraw(uint256 _amount) external;

    function getReward() external;

    function balanceOfToken(address account) external view returns (uint256);
}