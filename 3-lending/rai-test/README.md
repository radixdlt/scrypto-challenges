# RAI Scrypto Lending Platform Challenge Submission

This challenge submission is inspired by the impact made by the MakerDAO protocol on Ethereum. Providing trustless leverage and creating a decentralized stablecoin backed by assets is one of the first innovations that lead to the birth of Decentralized Finance (DeFi). 
In this scrypto challenge submission, borrowers are able to lock their XRD in a position as collateral for leverage to mint RAI, a token which can always be used to repay $1 of debt against a collateralized XRD position.
Note that the primary purpose of RAI is to generate leverage for a long XRD position. The demand for RAI as a stablecoin might outpace the generation of RAI from risktakers that mint RAI for leverage. The stablecoin aspect is reflected in the underlying value of RAI to be used to pay down $1 of debt in the protocol, and there is no market maker that attempts to peg the price of RAI to $1 on any trading markets.

# Features

- Follows the battle tested ratio of 150% overcollateralized positions introduced by MakerDAO
- Open positions and allow position management for borrowers - minting debt RAI tokens, paying off debt, closing positions, adding and withdrawing collateral
- Allows liquidators to liquidate undercollateralized positions
- Protocol Insolvency Fractional Redemption Strategy: When the protocol pooled collateral vault falls under 1:1 backing with RAI debt, the protocol freezes liquidations and new positions and allows fractional redemption strategy for RAI token holders. The guiding principle is that under extreme market conditions, the RAI token holders should not have to rush to the exit to redeem their collateral - instead, the collateral pool is split amongst all RAI token supply holders. All token holders will get their equivalent share of the collateral pool according to the supply of RAI that they hold. In this way, the fractional redemption strategy reduces volatity of the ecosystem and also stops prevents liquidations from flooding the market with XRD and crashing the supply of the XRD collateral. 
- Supports variable interest rates (locked to admin badge holders at the moment)
- Web Frontend UI utilizing the PTE Babylon Radix Browser extension to deploy blueprints, instantiate components, and interact with all functionality provided by the RAI Lending Platform
- Supports Oracle placeholder cross-blueprint pattern to allow users to test the behavior of the system under different market conditions (by changing the price of XRD on demand through the oracle contract)
- Convenience functions to print the state of global positions on the protocol to allow manual inspection and liquidation while waiting on the ability for the PTE to generate events that can be subscribed to for running liquidation bots
- Hosted web frontend available at https://rai-scrypto-lending-platform.dekentz.repl.co/

# General Usage

The general usage of the platform for borrowers looking to generate leverage on XRD position is as follows:

1. Open Position: Provide XRD collateral to be locked for a new position. Returned is a position badge NFT that grants authorization to manage the position. Most following position management functions require a Proof of the position badge to be passed to grant authorization to manage the position.
2. Draw: Determine amount of RAI token to be minted against the position. 
3. Paydown: Return an amount of RAI token to pay down debt on the position. The RAI is burned from supply.
4. AddCollateral: Add an additional amount of XRD collateral to the position to allow more RAI to be minted or to manage collateralization ratios.
5. PartialWithdrawCollateral: Withdraw amount of free XRD collateral not required to maintain the RAI position debt.
6. Close Position: Callable with a bucket of RAI, the function will pay off all accrued position debt and return the underlying collateral.

<img src="https://www.plantuml.com/plantuml/svg/bLRVYnit47xNNp7G59BMRhc7lfYIvhWZS93Imoca4F8mbiPUOQsqqGyxJkd_tf7MUzgzxb9oSB3MV3FpVVFDoduJ8lgOMYE4-FZ39rY_XZywieykwAYT5UCF6xYdBZ_3dJC68taqy0pnkxOHTgWE19uawIx2tdcmT8Rk2WAjLXoCoLk83YGQGvxRCvD8S2kZFj5G4FuMujd9MZ77Uu_dsS2jKfNSBEAHzMeq7tHi55VvN_H5kRCL8dVSmFl6JOH0vvtRhthPszk7AwnyzFGvKOYamAPsHnxSFaFWPUgoxXQ_mCvvpmMnTOaJNjpyzClcnGt89qe_C_xLq3ewJs0SMcwghTGT6a2hW3ed6o4cofUPebAmI_98iGo_gXTNgKkfDRwb3YzGL1H1bSwFmCB9i-eLXCANZL_ap1Yop0RbaGyluAb5RRKzbAfw3-v4laEjX5WqRPdjSiPmDb8FMLbnXGcqgHs7ziOvtpl6wlZ_gPluHqpM7zQyCTCj4ltcS8H3TyQ56hdESoQqJ1x-Yws-pySddpRm9nuf31imREJOmUWYcYSVNwrWxucoQSCxMqSpDh2UzjXbBwSJ-_YTysWFzEarlDEVarPSP0DtQ0nnvSlJeVD1L42HCNQzD2tBY9UMR8INCrjNHGrQp51HrM9LoRZXsyG910zjvrbAQR46JF9P2WHNAx6xv44oZmD5w7fJmPEYjki1Uo3k42-YjieZOGZFuz6xwBZqWmtEdCZAomOU0dJNOz3Ny_XRlhg7l7N--wRa_kQToU3RTwxCXuRig520P1NvEWFzbHJSR6xWt92diXvv4Ie0_jhUmmbD8lYrc1PIrvbBVzGlt5S2rr5PeEoSD2xavs6qePmwoqbeZ3kFXXfcEMzhlCzWYYUDt0kBXLDVGlJCzC1vjyOKXeqph34KCpEGMlYUxHkBEc4o4L1AvrNc6Xt4XhGlmB7CXeoQ9VbsbepIGRhayP3VMxH7yCd26gpZXz1Ssm1zqH6mY9iGFci-s17WpX1Z1_AL6J_EURH8S7bQ4wQTfxnTUM91OXd8yB8L4_RUjULgpQ72eKL_fCY4OtETr0hmv7G1I8-X6P1zTE1UETVVC7SDoUEWuFsLlvWzpV1j6SfyV0Fld3oE_2jVRLs_HSneyjicYBxKx3sx0eb6fams5EPb_EM-7PUlTsrPytn3eAbMbPVDNAfRTVdozz4RiYgrvby0">

For liquidators, they may use the following functions:
1. Print All Positions: This function returns a list of all current positions at the protocol in the info logs of the Radix Engine. The Web Frontend UI displays the logs, and allows liquidators to manually inspect which positions are available to be liquidated.
2. Liquidate: Providing a position id and RAI bucket, pay off the RAI debt of the undercollateralized position in exchange for foreclosing on the underlying collateral position. Liquidators are incentivized to monitor positions and call the liquidate function as they may be able to "purchase" the underlying collateral at a discount by paying off the loan.

For each liquidate call, there will be a check to see if the protocol remains solvent ($1 collateral for each $1 RAI token). Under extreme market conditions when the protocol becomes insolvent ($collateral < $RAI debt), the protocol triggers redemption only mode - allowing RAI holders to always be able to claim their portion of the collateral pool, and limiting the risk of a bank run and cascading liquidations. At that time, the redeem function will be open for RAI holders to exchange their RAI for their share of the XRD collateral pool.
At any time, anyone can call the check_protocol_solvency function, and it is not necessary for a liquidation to happen to trigger the protocol into fractional redemption mode.

# Deployment steps

The instantiation of the RAI Lending Platform requires a oracle to provide the price of XRD for collateral. In order to instantiate the platform:

1. Publish an oracle blueprint to the ledger (this submission includes an OraclePlaceholder contract in dependencies/oracle_placeholder/src/lib.rs)
2. Instantiate an oracle component from the blueprint (and interact with this new component to set/get the price of xrd used for calculations in the contract)
3. Publish the RAI Lending Platform blueprint to the ledger
4. Instiate the RAI Lending Platform component, passing it in the ComponentAddress of the instantiated oracle from step 2. 

# Web Frontend UI

The web frontend UI is provided in a git submodule in this folder. A hosted version of the web frontend is available at https://rai-scrypto-lending-platform.dekentz.repl.co/
Please note that the Babylon PTE Browser Extension is required to interact with the RAI Lending Platform with a test execution environment.

To access a local copy of the Web Frontend UI source code, 
1. git submodule init
2. git submodule update
This should fetch the source code of the Web Frontend to the local environment. 

![image](https://user-images.githubusercontent.com/104961484/179461540-644e6574-04d4-4d45-94ac-70204937a2e6.png)

The Web UI provides convenience functions for users to be able to interact with the oracle xrd price, and view the amount of XRD, RAI, and Position Badges they hold at the moment. The main way that users can keep track of events happening on ledger is through reading the logs generated by each function. At the bottom of the web page, an "All Logs: " section keeps track of all function calls that have happened in the current session.
It is expected that blueprints and components will not be redeployed often, and for user-experience friendliness, the package address and component addresses are stored in browser localstorage to make it easier/faster to interact with the protocol across browser refreshes.

# Directory Structure
- dependencies/oracle_placeholder - a small oracle contract used for cross-blueprint oracle price check functionality
- RAI-Scrypto-Lending-Platform-PTE - a react web frontend for testing with Babylon PTE, tracked in a separate git repo. It may be cloned into the local repo by using the git submodule commands above.
- src/lib.rs - core RAI Lending Platform logic
- raitest.rev - revup script for testing RAI Lending Platform functions with resim
