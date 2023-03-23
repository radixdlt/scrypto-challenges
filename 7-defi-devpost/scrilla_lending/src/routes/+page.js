// @ts-nocheck
import {
    ManifestBuilder,
    Decimal, Expression,
    ResourceAddress,
    RadixDappToolkit,
    Bucket,
    Proof,
    NonFungibleId,
    ComponentAddress
} from '@radixdlt/radix-dapp-toolkit'

import { TransactionApi, StateApi, StatusApi, StreamApi } from "@radixdlt/babylon-gateway-api-sdk";

// Instantiate Gateway SDK
const transactionApi = new TransactionApi();
const stateApi = new StateApi();
const statusApi = new StatusApi();
const streamApi = new StreamApi();

const rdt = RadixDappToolkit(
    {
        dAppDefinitionAddress:
        'account_tdx_b_1pq40mdn54g2w7pddj4mj4k6djpzye579tx9k68l5hcdqvmtyn7',
        dAppName: 'Scrilla Lending',
    },
    (requestData) => {
        requestData({
            accounts: { quantifier: 'atLeast', quantity: 1 },
        }).map(({ data: { accounts } }) => {
            // set your application state
            let accountName = accounts[0].label
            sessionStorage.setItem('accountName',accountName)
            let accountAddress = accounts[0].address
            sessionStorage.setItem('accountAddress',accountAddress)
            location.reload();
        })
    },
    {
        networkId: 11,
        onDisconnect: () => {
        // clear your application state
          sessionStorage.clear();
          location.reload();
        },
        onInit: ({ accounts }) => {
            // set your initial application state
            if (accounts.length > 0 ) {
                let accountName = accounts[0].label
                sessionStorage.setItem('accountName',accountName)
                let accountAddress = accounts[0].address
                sessionStorage.setItem('accountAddress',accountAddress)
            }
        },
    }
)

export const _sendManifest = (manifest) => {
    return new Promise(async (resolve, reject) => {
        // Send manifest to extension for signing //
        const res = await rdt
        .sendTransaction({
            transactionManifest: manifest,
            version: 1,
        })

        if (res.isErr()) {
            console.log(res.error);
            reject(res.error)
        } 

        console.log("Instantiate WalletSDK Result: ", res.value)

        // ************ Fetch the transaction status from the Gateway API ************
        let status = await transactionApi.transactionStatus({
            transactionStatusRequest: {
                intent_hash_hex: res.value.transactionIntentHash
            }
        });
        console.log('Instantiate TransactionApi transaction/status:', status)

        // ************* fetch component address from gateway api and set componentAddress variable **************
        let commitReceipt = await transactionApi.transactionCommittedDetails({
            transactionCommittedDetailsRequest: {
                transaction_identifier: {
                    type: 'intent_hash',
                    value_hex: res.value.transactionIntentHash
                }
            }
        })
        console.log('Instantiate Committed Details Receipt', commitReceipt)
        resolve(commitReceipt);
    })
}

export const _instantiateComponent = async () => {
    let packageAddress = 'package_tdx_b_1q8sa7nw0xdyh9ya9jlmzjwcestz8cljth9wjg6kf8daqvadky2';
    let accountAddress = sessionStorage.getItem('accountAddress');

    let manifest = new ManifestBuilder()
        .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")]) // for some reason need this to call funds. 
        .callFunction(packageAddress, "Scrilla", "instantiate_scrilla_module", [])
        .callMethod(accountAddress, "deposit_batch", [Expression("ENTIRE_WORKTOP")])
        .build()
        .toString();
    console.log("Instantiate Manifest: ", manifest)

    // call helper function to send manifest to network
    _sendManifest(manifest)
    .then(commitReceipt => {
        // ****** set componentAddress and resourceAddress variables with gateway api commitReciept payload ******
        // componentAddress = commitReceipt.details.receipt.state_updates.new_global_entities[0].global_address <- long way -- shorter way below ->
        let componentAddress1 = commitReceipt.details.referenced_global_entities[0]
        sessionStorage.setItem('componentAddress1',componentAddress1);
        let componentAddress2 = commitReceipt.details.referenced_global_entities[1]
        sessionStorage.setItem('componentAddress2',componentAddress2);
        let componentAddress3 = commitReceipt.details.referenced_global_entities[2]
        sessionStorage.setItem('componentAddress3',componentAddress3);
        /*
        0 PriceOracle
        "component_tdx_b_1qg4g0j6kh93yxuzevht5zwumadccl24q95c277t33cxq6szqxg"
        1 UserManagement
        "component_tdx_b_1q2n5pgg78va7w0v7y6vfmdktwfh4wdfdn878an5x8mlsyyg7ya"
        2 Scrilla
        "component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9"
        3 Scrilla Admin
        "resource_tdx_b_1qq4g0j6kh93yxuzevht5zwumadccl24q95c277t33cxqs3swrn"
        4 Scrilla Protocol Badge
        "resource_tdx_b_1qqhz082puqqd9rhat5zuaykz2ecawy42psqk4cruusws3zuf89"
        5 Scrilla User
        "resource_tdx_b_1qpye3s55d9yx44qsqehh5r4fa42f82jttzsy83wr2xhqgt7ms5" 
        6 Auth Badge
        "resource_tdx_b_1qzn5pgg78va7w0v7y6vfmdktwfh4wdfdn878an5x8mlsw96spx"
        7 Scrilla Token | SCRL
        "resource_tdx_b_1qrzslz5u8stkmcm4vxwftll4m9wxhmwk0pgscmfdm62q0ds27r"
        8 USD-Scrilla | USDS
        "resource_tdx_b_1qrr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqttv7n7"
            */
        // location.reload();
    })
}

// export const _showInfo = async (accountAddress,scrillaComponentAddress) => {
//     let manifest = new ManifestBuilder()
//         .callMethod(scrillaComponentAddress,"show_info",[])
//         .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
//         .build()
//         .toString()
    
//     _sendManifest(manifest)
//     .then(commitReceipt => {
//         console.log(commitReceipt);
//     })
// }

export const _addUsdsToShield = async (accountAddress, scrillaUserNftAddress, amountToDeposit, usdsAddress, scrillaComponentAddress) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")]) // for some reason need this to call funds. 
            // first nft to take to provide as proof and will burn
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(1),ResourceAddress(scrillaUserNftAddress)])
            // puts that nft into a bucket
            .takeFromWorktop(scrillaUserNftAddress,"user_nft_bucket")
            // create a proof from the bucket
            .createProofFromBucket("user_nft_bucket", "user_nft_proof")
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(amountToDeposit),ResourceAddress(usdsAddress)])
            .takeFromWorktop(usdsAddress,"usds_bucket")
            .callMethod(scrillaComponentAddress,"deposit_to_shield_pool",[Bucket("usds_bucket"),Proof("user_nft_proof")])
            .returnToWorktop("user_nft_bucket")
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();
    
        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        })
        .catch(err => {
            reject(err);
        })
    })
    
}

export const _addXrdToCollateral = async (accountAddress, scrillaUserNftAddress, amountToDeposit, scrillaComponentAddress) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
            // first nft to take to provide as proof and will burn
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(1),ResourceAddress(scrillaUserNftAddress)])
            // // put that nft into a bucket
            .takeFromWorktop(scrillaUserNftAddress,"user_nft_bucket")
            // // create a proof from the bucket
            .createProofFromBucket("user_nft_bucket","user_nft_proof")
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(amountToDeposit),ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")])
            .takeFromWorktop("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp","xrd_bucket")
            .callMethod(scrillaComponentAddress,"add_xrd_to_collateral",[Bucket("xrd_bucket"),Proof("user_nft_proof")])
            // // return a bucket to worktop
            .returnToWorktop("user_nft_bucket")
            // // return tokens to accoutn from worktop
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();

        _sendManifest(manifest)
            .then(commitReceipt => {
                resolve(commitReceipt);
            })
            .catch(err => {
                reject(err);
            })
    })
}

export const _borrowUsds = async (accountAddress,scrillaUserNftAddress,scrillaComponentAddress,amountToBorrow) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")]) // for some reason need this to call funds. 
            // first nft to take to provide as proof and will burn
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(1),ResourceAddress(scrillaUserNftAddress)])
            // puts that nft into a bucket
            .takeFromWorktop(scrillaUserNftAddress,"user_nft_bucket")
            // create a proof from the bucket
            .createProofFromBucket("user_nft_bucket","user_nft_proof")
            .callMethod(scrillaComponentAddress,"borrow_usds",[Decimal(amountToBorrow),Proof("user_nft_proof")])
            // return a bucket to worktop
            .returnToWorktop("user_nft_bucket")
            // return tokens to account from worktop
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();
    
        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        })
        .catch(err => {
            reject(err);
        })
    })
    
}

export const _callLiquidiation = async (scrillaComponentAddress, ) => {
    let manifest = new ManifestBuilder()
        .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")]) // for some reason need this to call funds. 
        .callMethod(scrillaComponentAddress,"call_liquidation",[NonFungibleId("#3#")]) // couldnt find 'NonFungibleLocalId' as in the .rtm 
        .build()
        .toString();

    _sendManifest(manifest)
    .then(commitReceipt => {
        console.log(commitReceipt);
    })
}

export const _newUser = async (scrillaComponentAddress,accountAddress) => {
    return new Promise((resolve, reject) => {
        let manifest = new ManifestBuilder()
        .callMethod(scrillaComponentAddress, "new_user",[])
        .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
        .build()
        .toString();

        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        })
        .catch(err => {
            reject(err);
        })
    })

}

export const _redeemUsds = async (accountAddress, amountToRedeem, usdsAddress, scrillaComponentAddress) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")]) // for some reason need this to call funds. 
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(amountToRedeem),ResourceAddress(usdsAddress)])
            .takeFromWorktopByAmount(amountToRedeem,usdsAddress,"bucket1")
            .callMethod(scrillaComponentAddress,"redeem_usds",[Bucket("bucket1")])
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();

        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        })
        .catch(err => {
            reject(err);
        })
    })
    
}

export const _removeXrdFromCollateral = async (accountAddress,scrillaUserNftAddress,scrillaComponentAddress,amountToRemove) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
            // first nft to take to provide as proof and will burn
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(1),ResourceAddress(scrillaUserNftAddress)])
            // puts that nft into a bucket
            .takeFromWorktop(scrillaUserNftAddress,"user_nft_bucket")
            // create a proof from the bucket
            .createProofFromBucket("user_nft_bucket","user_nft_proof")
            .callMethod(scrillaComponentAddress,"remove_xrd_from_collateral",[Decimal(amountToRemove),Proof("user_nft_proof")])
            // return a bucket to worktop
            .returnToWorktop("user_nft_bucket")
            // return tokens to account from worktop
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();

        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        })  
        .catch(err => {
            reject(err);
        })
    })
     
}

export const _repayUsds = async (accountAddress,scrillaUserNftAddress,amountToRepay,usdsAddress,scrillaComponentAddress) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")]) // for some reason need this to call funds. 
            // first nft to take to provide as proof and will burn
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(1),ResourceAddress(scrillaUserNftAddress)])
            // puts that nft into a bucket
            .takeFromWorktop(scrillaUserNftAddress,"user_nft_bucket")
            //create a proof from the bucket
            .createProofFromBucket("user_nft_bucket","user_nft_proof")
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(amountToRepay),ResourceAddress(usdsAddress)])
            .takeFromWorktop(usdsAddress,"usds_bucket")
            .callMethod(scrillaComponentAddress,"repay_usds_loan",[Bucket("usds_bucket"),Proof("user_nft_proof")])
            // return a bucket to worktop
            .returnToWorktop("user_nft_bucket")
            // return tokens to account from worktop
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();

        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        }) 
        .catch(err => {
            reject(err);
        })
    })
      
}

export const _setPrice = async (scrillaComponentAddress,setPrice,accountAddress) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
        .callMethod(scrillaComponentAddress,"set_price",[Decimal(setPrice)])
        .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
        .build()
        .toString();

        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        })  
        .catch(err => {
            reject(err);
        })
    })
}

export const _getPrice = async (scrillaComponentAddress,accountAddress) => {
    return new Promise((resolve, reject) => {
        let manifest = new ManifestBuilder()
            .callMethod(scrillaComponentAddress,"get_xrd_price",[])
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();

        _sendManifest(manifest)
        .then(commitReceipt => {
            console.log(commitReceipt);
            resolve(commitReceipt.details.receipt.output[1].data_json)
        })
        .catch(err => {
            reject(err);
        })
    })

}

export const _withdrawShieldDepositAndRewards = async (accountAddress,scrillaUserNftAddress,scrillaComponentAddress) => {
    return new Promise((resolve,reject) => {
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")]) // for some reason need this to call funds. 
            // first nft to take to provide as proof and will burn
            .callMethod(accountAddress,"withdraw_by_amount",[Decimal(1),ResourceAddress(scrillaUserNftAddress)])
            // puts that nft into a bucket
            .takeFromWorktop(scrillaUserNftAddress,"user_nft_bucket")
            // create a proof from the bucket
            .createProofFromBucket("user_nft_bucket","user_nft_proof")
            .callMethod(scrillaComponentAddress,"withdraw_shield_deposit_and_rewards",[Proof("user_nft_proof")])
            // return a bucket to worktop
            .returnToWorktop("user_nft_bucket")
            // return tokens to account from workto
            .callMethod(accountAddress,"deposit_batch",[Expression("ENTIRE_WORKTOP")])
            .build()
            .toString();

        _sendManifest(manifest)
        .then(commitReceipt => {
            resolve(commitReceipt);
        }) 
        .catch(err => {
            reject(err);
        })
    })
     
}