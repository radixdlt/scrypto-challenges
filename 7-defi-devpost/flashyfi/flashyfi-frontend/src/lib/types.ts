import type {ComponentAddressString, ResourceAddressString, SdkError} from "@radixdlt/radix-dapp-toolkit";
import type {TransactionStatusResponse} from "@radixdlt/babylon-gateway-api-sdk/dist/generated/models";
import {shortenAddress} from "./utils";
import type {
    FungibleResourcesCollectionItem,
    NonFungibleResourcesCollectionItem
} from "@radixdlt/babylon-gateway-api-sdk";

export type FlashyfiAddresses = {
    flashyfiBadgeResource: ResourceAddressString,
    accountConfigBadgeResource: ResourceAddressString,
    loanReceiptResource: ResourceAddressString,
}

export type FlashyfiAccount = {
    accountAddress: ComponentAddressString,
    fungibleFeeConfigs: Array<FeeConfig>,
    nonFungibleFeeConfigs: Array<FeeConfig>,
    availableFungibleResources: Map<ResourceAddressString, FungibleResourcesCollectionItem>,
    availableNonFungibleResources: Map<ResourceAddressString, NonFungibleResourcesCollectionItem>,
    withdrawMethodsAreAccessible: boolean
}

export type FungibleAmountFee = {
    resourceAddress: string,
    amount: string
}
export type PercentFee = {
    value: number
}
export type Fee = FungibleAmountFee | PercentFee

export type SendTransactionErrorResponse = SdkError | TransactionStatusResponse | any

export class ResourceDetails {
    constructor(
        public readonly address: ResourceAddressString,
        public readonly metadata: Map<string, string>,
        public readonly fungible: boolean
    ) {
    }

    get symbol(): string | undefined {
        return this.metadata.get("symbol")
    }

    get name(): string | undefined {
        return this.metadata.get("name")
    }

    get shortenedAddress(): string {
        return shortenAddress(this.address)
    }

    getSymbolOrNameOrShortenedAddress(maxLength: number = 15): string {
        return ResourceDetails.shorten(this.symbol, maxLength)
            ?? ResourceDetails.shorten(this.name, maxLength)
            ?? this.shortenedAddress
    }

    hasSymbolOrName(): boolean {
        return this.symbol != undefined || this.name != undefined
    }

    getLabel(maxLength: number = 66): string {
        if (this.hasSymbolOrName()) {
            const shortenedSymbolOrName = this.getSymbolOrNameOrShortenedAddress(maxLength - this.shortenedAddress.length);
            return `${shortenedSymbolOrName} (${this.shortenedAddress})`
        }

        if (maxLength >= this.address.length) {
            return this.address
        } else {
            const ellipsisLength = 3
            const startLength = maxLength - Math.floor(maxLength / 3)
            return shortenAddress(this.address, startLength, maxLength - startLength - ellipsisLength)
        }
    }

    private static shorten(text: string | undefined, maxLength: number): string | undefined {
        if (text == undefined) return undefined

        if (text.length > maxLength) {
            return text.slice(0, text.length - 3) + "..."
        } else {
            return text
        }
    }
}

// Attention: Keep the order of the enum in sync with the Fee enum in Scrypto!
export enum FeeType {
    PERCENTAGE = "Percentage Fee",
    FIXED = "Fixed Fee",
}

export type FeeConfig = {
    enabled: boolean,
    resourceAddress: ResourceAddressString,
    feeType?: FeeType,
    feeValue?: number
}
