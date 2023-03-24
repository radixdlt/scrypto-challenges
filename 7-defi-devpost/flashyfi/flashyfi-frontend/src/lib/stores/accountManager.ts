import {derived, type Readable, type Writable, writable} from "svelte/store";
import type {FlashyfiAccount} from "../types";
import {FeeType} from "../types";
import type {Account, ComponentAddressString} from "@radixdlt/radix-dapp-toolkit";

class AccountManager {
    public readonly connectedAccounts: Readable<Account[]> = writable([])
    public readonly allFlashyfiAccountsPromise: Readable<Promise<Array<FlashyfiAccount>>> = writable(Promise.resolve([]))

    public readonly connectedAccountAddresses = derived(
        this.connectedAccounts,
        $connectedAccounts => new Set($connectedAccounts.map(account => account.address))
    )

    public readonly connectedFlashyfiAccountConfigsPromise = derived(
        [this.connectedAccountAddresses, this.allFlashyfiAccountsPromise],
        ([$connectedAccounts, $allFlashyfiAccountsPromise]) =>
            $allFlashyfiAccountsPromise.then(flashyfiAccounts => {
                const accounts = new Map<ComponentAddressString, FlashyfiAccount>()
                flashyfiAccounts.forEach(flashyfiAccount => {
                    if ($connectedAccounts.has(flashyfiAccount.accountAddress)) {
                        accounts.set(flashyfiAccount.accountAddress, flashyfiAccount)
                    }
                })
                return accounts
            })
    )

    public readonly borrowableResourcesPromise = derived(
        this.allFlashyfiAccountsPromise,
        $allFlashyfiAccountsPromise =>
            $allFlashyfiAccountsPromise.then(flashyfiAccounts => {
                const fungibleResources = new Set(flashyfiAccounts
                    .flatMap(account =>
                        account.fungibleFeeConfigs
                            .filter(config => config.enabled)
                            // This demo only supports fixed fees!
                            .filter(config => config.feeType === FeeType.FIXED)
                            .flatMap(config => config.resourceAddress)
                    ))
                const nonFungibleResources = new Set(flashyfiAccounts
                    .flatMap(account =>
                        account.nonFungibleFeeConfigs
                            .filter(config => config.enabled)
                            .flatMap(config => config.resourceAddress)
                    ))

                return [...fungibleResources, ...nonFungibleResources]
            })
    )

    setConnectedAccounts(accounts: Account[]) {
        (this.connectedAccounts as Writable<Account[]>).set(accounts)
    }

    setAllFlashyfiAccounts(flashyfiAccountsPromise: Promise<FlashyfiAccount[]>) {
        (this.allFlashyfiAccountsPromise as Writable<Promise<Array<FlashyfiAccount>>>).set(flashyfiAccountsPromise)
    }
}

export default new AccountManager()