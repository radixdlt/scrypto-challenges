import { useState, useEffect } from "react";
import { useRdt } from "./useRdt.js";
import { useAccount } from "./useAccount.jsx";
import {getNftDataFromMongo} from "../api/get.js";


export const useCombinedNftData = (YieldNftRaddy) => {
    const [enhancedNfts, setEnhancedNfts] = useState([]);
    const { accounts } = useAccount();
    const gatewayApi = useRdt().gatewayApi;

    useEffect(() => {
        const fetchAndProcessAccounts = async () => {
            if (!gatewayApi || !accounts || accounts.length === 0 || !YieldNftRaddy) {
                setEnhancedNfts([]);
                return;
            }

            try {
                const addresses = accounts.map(account => account.address);
                const accountInfos = await gatewayApi.state.getEntityDetailsVaultAggregated(addresses, {
                    nonFungibleIncludeNfids: true
                });

                if (accountInfos.length > 0) {

                    const preProcessedAccounts = accountInfos.map(account => ({
                        address: account.address,
                        non_fungible_resources: account.non_fungible_resources
                    }));

                    const processedAccounts = await Promise.all(preProcessedAccounts.map(async (account) => {
                        const nfts = account.non_fungible_resources.items
                            .filter(item => item.resource_address === YieldNftRaddy)
                            .flatMap(nft =>
                                nft.vaults && nft.vaults.items.length > 0
                                    ? nft.vaults.items.flatMap(vault =>
                                        vault.items.map(id => ({
                                            label: id.replace(/#/g, ''),
                                            value: id
                                        }))
                                    )
                                    : []
                            );

                        const nftsWithData = await Promise.all(
                            nfts.map(async nft => {
                                const additionalData = await getNftDataFromMongo(nft.label);
                                return {
                                    ...nft,
                                    data: {
                                        hour_of_mint: additionalData.hour_of_mint,
                                        n_super_minted: additionalData.n_super_minted,
                                        n_trust_minted: additionalData.n_trust_minted["$numberDecimal"],
                                        createdAt: additionalData.createdAt
                                    }
                                };
                            })
                        );

                        return { address: account.address, nfts: nftsWithData };
                    }));

                    setEnhancedNfts(processedAccounts);
                } else {
                    setEnhancedNfts([]);
                }
            } catch (error) {
                console.error("Error in fetching or processing account information:", error);
            }
        };

        fetchAndProcessAccounts();
    }, [accounts, YieldNftRaddy, gatewayApi]);

    return enhancedNfts;
};