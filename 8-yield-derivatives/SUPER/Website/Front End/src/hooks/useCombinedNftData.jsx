import { useState, useEffect } from "react";
import { useRdt } from "./useRdt.js";
import { useAccount } from "./useAccount.jsx";
import {getNftDataFromMongo} from "../api/get.js";

/**
 * Custom hook to fetch and process NFT data for all accounts connected to the frontend.
 *
 * @param {string} YieldNftRaddy - The resource address of the yield NFT.
 * @returns {Array} The combined NFT data.
 */
export const useCombinedNftData = (YieldNftRaddy) => {

    // State to hold the enhanced NFT data
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
                // Get addresses from accounts
                const addresses = accounts.map(account => account.address);

                // Fetch account details from the Gateway API
                const accountInfos = await gatewayApi.state.getEntityDetailsVaultAggregated(addresses, {
                    nonFungibleIncludeNfids: true
                });

                if (accountInfos.length > 0) {

                    const preProcessedAccounts = accountInfos.map(account => ({
                        address: account.address,
                        non_fungible_resources: account.non_fungible_resources
                    }));

                    // Process each account to fetch NFT data and combine with backend-fetched data
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

                        // Fetch additional data from backend for each NFT
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