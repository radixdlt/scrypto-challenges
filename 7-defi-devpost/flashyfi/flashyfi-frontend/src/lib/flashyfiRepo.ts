import {
    type FungibleResourcesCollectionItem,
    type GatewayInformationResponseAllOfWellKnownAddresses,
    type NonFungibleResourcesCollectionItem,
    StateApi,
    StatusApi
} from "@radixdlt/babylon-gateway-api-sdk";
import type {ComponentAddressString, ResourceAddressString} from "@radixdlt/radix-dapp-toolkit";
import {
    Array,
    Bool,
    ComponentAddress,
    Decimal,
    ManifestBuilder,
    Proof,
    ResourceAddress,
    Tuple,
} from "@radixdlt/radix-dapp-toolkit";
import {FLASHYFI_COMPONENT_ADDRESS, NETWORK_ID_BETANET} from "./constants";
import type {
    EntityDetailsResponseComponentDetails
} from "@radixdlt/babylon-gateway-api-sdk/dist/generated/models/EntityDetailsResponseComponentDetails";
import type {FeeConfig, FlashyfiAccount, FlashyfiAddresses} from "$lib/types";
import {FeeType, ResourceDetails} from "./types";
import type {NonFungibleIdsResponse} from "@radixdlt/babylon-gateway-api-sdk/dist/generated/models";
import RadixEngineToolkit from "./radixEngineToolkit";
import {base} from "$app/paths";

class FlashyfiRepo {
    private stateApi = new StateApi();
    private statusApi = new StatusApi();
    private readonly radixEngineToolkitPromise: Promise<RadixEngineToolkit>
    private flashyfiAddresses: FlashyfiAddresses | null = null
    private well_known_addresses: GatewayInformationResponseAllOfWellKnownAddresses | null = null
    private resourceDetails: Map<ResourceAddressString, ResourceDetails> = new Map()

    constructor() {
        this.radixEngineToolkitPromise = WebAssembly.instantiateStreaming(fetch(base+ "/wasm/radix_engine_toolkit.wasm"))
            .then(wasm => new RadixEngineToolkit(wasm.instance))
    }

    async getFlashyfiAddresses(): Promise<FlashyfiAddresses> {
        if (this.flashyfiAddresses == null) {
            const response = await this.stateApi.entityDetails({
                entityDetailsRequest: {
                    address: FLASHYFI_COMPONENT_ADDRESS
                }
            })
            const details = response.details as EntityDetailsResponseComponentDetails
            // @ts-ignore
            const addresses: Array<ResourceAddressString> = details.state.data_json[1]
            this.flashyfiAddresses = {
                flashyfiBadgeResource: addresses[0],
                accountConfigBadgeResource: addresses[1],
                loanReceiptResource: addresses[2],
            }
        }

        return this.flashyfiAddresses!
    }

    async getWellKnownAddresses(): Promise<GatewayInformationResponseAllOfWellKnownAddresses> {
        if (this.well_known_addresses == null) {
            const information = await this.statusApi.gatewayInformation()
            this.well_known_addresses = information.well_known_addresses
        }
        return this.well_known_addresses!
    }

    async getAllAccountResources(account_address: ComponentAddressString): Promise<AccountResources> {
        let fungibles: FungibleResourcesCollectionItem[] = []
        let nonFungibles: NonFungibleResourcesCollectionItem[] = []

        const response = await this.stateApi.entityResources({
            entityResourcesRequest: {
                address: account_address
            }
        })
        response.fungible_resources.items.forEach(item => fungibles.push(item))
        response.non_fungible_resources.items.forEach(item => nonFungibles.push(item))

        let nextCursor = response.fungible_resources.next_cursor
        while (nextCursor != null) {
            const response = await this.stateApi.entityFungibles({
                entityFungiblesRequest: {
                    address: account_address
                }
            })
            response.fungibles.items.forEach(item => fungibles.push(item))
            nextCursor = response.fungibles.next_cursor
        }

        nextCursor = response.non_fungible_resources.next_cursor
        while (nextCursor != null) {
            const response = await this.stateApi.entityNonFungibles({
                entityNonFungiblesRequest: {
                    address: account_address
                }
            })
            response.non_fungibles.items.forEach(item => nonFungibles.push(item))
            nextCursor = response.non_fungibles.next_cursor
        }

        return {fungibles, nonFungibles}
    }

    async getResourceDetails(resourceAddress: ResourceAddressString): Promise<ResourceDetails> {
        if (!this.resourceDetails.has(resourceAddress)) {
            const response = await this.stateApi.entityDetails({
                entityDetailsRequest: {
                    address: resourceAddress
                }
            })
            const metadata: Map<string, string> = new Map()
            for (const {key, value} of response.metadata.items) {
                metadata.set(key, value)
            }
            const fungible = response.details?.discriminator === "fungible_resource"
            this.resourceDetails.set(resourceAddress, new ResourceDetails(resourceAddress, metadata, fungible))
        }

        return this.resourceDetails.get(resourceAddress)!
    }


    async createManifestFlashyfiAccount(
        accountAddress: ComponentAddressString,
        fungibleFeeConfigs: Array<FeeConfig>,
        nonFungibleFeeConfigs: Array<FeeConfig>
    ) {
        const publicKeyGlobalId = await this.getAccountPublicKeyGlobalId(accountAddress);
        const addresses = await this.getFlashyfiAddresses()

        const manifestBuilder = new ManifestBuilder()
            .callMethod(FLASHYFI_COMPONENT_ADDRESS, "flashyfi_account", [ComponentAddress(accountAddress),])

        this.addUpdateAccountConfigInstructionsToManifest(manifestBuilder, accountAddress, addresses, fungibleFeeConfigs, nonFungibleFeeConfigs)
        let manifest = manifestBuilder.build().toString();

        for (const method of ["withdraw_by_amount", "withdraw_by_ids"]) {
            const instruction = `
            SET_METHOD_ACCESS_RULE
                ComponentAddress("${accountAddress}")
                1u32
                Enum(0u8, "${method}")
                Enum(2u8, Enum(1u8, Array<Enum>(Enum(0u8, Enum(0u8, Enum(0u8, NonFungibleGlobalId("${publicKeyGlobalId.resourceAddress}:${publicKeyGlobalId.localId}")))), Enum(0u8, Enum(0u8, Enum(1u8, ResourceAddress("${addresses.flashyfiBadgeResource}")))))))
                ;
            `
            manifest = instruction + "\n" + manifest
        }
        return manifest
    }

    async createManifestUpdateFlashyfiAccount(
        accountAddress: ComponentAddressString,
        fungibleFeeConfigs: Array<FeeConfig>,
        nonFungibleFeeConfigs: Array<FeeConfig>
    ) {
        const addresses = await this.getFlashyfiAddresses()
        let manifest = new ManifestBuilder()
        this.addUpdateAccountConfigInstructionsToManifest(manifest, accountAddress, addresses, fungibleFeeConfigs, nonFungibleFeeConfigs)
        return manifest.build().toString()
    }

    private addUpdateAccountConfigInstructionsToManifest(
        manifest: ManifestBuilder, accountAddress: ComponentAddressString, flashyfiAddresses: FlashyfiAddresses,
        fungibleFeeConfigs: Array<FeeConfig>, nonFungibleFeeConfigs: Array<FeeConfig>) {

        manifest.createProofFromAccount(accountAddress, flashyfiAddresses.accountConfigBadgeResource)
            .popFromAuthZone("account_config_badge")
            .callMethod(FLASHYFI_COMPONENT_ADDRESS, "update_account_config", [
                Proof("account_config_badge"),
                `Map<ResourceAddress, Tuple>(${
                    fungibleFeeConfigs
                        .filter(feeConfig => this.isFeeConfigValid(feeConfig))
                        .map(feeConfig => ResourceAddress(feeConfig.resourceAddress) + ", " + this.fungibleFeeConfigToManifestArg(feeConfig)).join(", ")
                })`,
                `Map<ResourceAddress, Tuple>(${
                    nonFungibleFeeConfigs
                        .filter(feeConfig => this.isFeeConfigValid(feeConfig))
                        .map(feeConfig => ResourceAddress(feeConfig.resourceAddress) + ", " + this.nonFungibleFeeConfigToManifestArg(feeConfig)).join(", ")
                })`
            ])
    }

    private isFeeConfigValid(feeConfig: FeeConfig): boolean {
        return feeConfig.feeType != null && feeConfig.feeValue != null && feeConfig.feeValue >= 0
    }

    private fungibleFeeConfigToManifestArg(feeConfig: FeeConfig): string {
        let fee: string;
        if (feeConfig.feeType === FeeType.PERCENTAGE) {
            fee = Decimal(feeConfig.feeValue!)
        } else if (feeConfig.feeType === FeeType.FIXED) {
            fee = Tuple(Decimal(feeConfig.feeValue!))
        } else {
            throw Error("Unhandled FeeType: " + feeConfig.feeType)
        }

        return Tuple(
            Bool(feeConfig.enabled),
            `Enum(${Object.values(FeeType).indexOf(feeConfig.feeType)}u8, ${fee})`
        )
    }

    private nonFungibleFeeConfigToManifestArg(feeConfig: FeeConfig): string {
        return Tuple(Bool(feeConfig.enabled), Tuple(Decimal(feeConfig.feeValue!)))
    }

    async getAllFlashyfiedAccounts(): Promise<FlashyfiAccount[]> {
        const promisedAccounts: Array<Promise<FlashyfiAccount>> = []
        const addresses = await this.getFlashyfiAddresses()

        let cursor: string | null | undefined = null
        do {
            const response: NonFungibleIdsResponse = await this.stateApi.nonFungibleIds({
                nonFungibleIdsRequest: {
                    address: addresses.accountConfigBadgeResource,
                    cursor
                }
            })
            cursor = response.non_fungible_ids.next_cursor
            for (const item of response.non_fungible_ids.items) {
                const account: Promise<FlashyfiAccount> = this.loadFlashyfiedAccountById(item.non_fungible_id)
                promisedAccounts.push(account)
            }
        } while (cursor)

        let accounts: Array<FlashyfiAccount> = []
        for (const promisedAccount of promisedAccounts) {
            accounts.push(await promisedAccount)
        }

        return accounts
    }

    private async loadFlashyfiedAccountById(nonFungibleId: string): Promise<FlashyfiAccount> {
        const addresses = await this.getFlashyfiAddresses()
        let response = await this.stateApi.nonFungibleIdData({
            nonFungibleDataRequest: {
                address: addresses.accountConfigBadgeResource,
                non_fungible_id: nonFungibleId
            }
        })
        const immutableData = await this.decodedSborData(response.immutable_data_hex)
        const accountAddress = immutableData.elements[0].address

        const mutableData = await this.decodedSborData(response.mutable_data_hex)
        const fungibleFeeConfigs = mutableData.elements[0].entries
            .map((entry: any) => {
                const key = entry[0]
                const value = entry[1]
                const [feeType, feeValue] = this.parseFeeEnumNode(value.elements[1])
                const feeConfig: FeeConfig = {
                    resourceAddress: key.address,
                    enabled: value.elements[0].value,
                    feeType,
                    feeValue
                }

                return feeConfig
            })
            .sort((feeConfig: FeeConfig) => feeConfig.enabled ? -1 : 1)
        const nonFungibleFeeConfigs = mutableData.elements[1].entries
            .map((entry: any) => {
                const key = entry[0]
                const value = entry[1]
                const feeConfig: FeeConfig = {
                    resourceAddress: key.address,
                    enabled: value.elements[0].value,
                    feeType: FeeType.FIXED,
                    feeValue: value.elements[1].elements[0].value
                }

                return feeConfig
            })
            .sort((feeConfig: FeeConfig) => feeConfig.enabled ? -1 : 1)

        const {fungibles, nonFungibles} = await this.getAllAccountResources(accountAddress)
        const withdrawMethodsAreAccessible = await this.areAccountsWithdrawMethodsAccessible(accountAddress);

        return {
            accountAddress, fungibleFeeConfigs, nonFungibleFeeConfigs,
            availableFungibleResources: new Map(fungibles.map(item => [item.address as ResourceAddressString, item])),
            availableNonFungibleResources: new Map(nonFungibles.map(item => [item.address as ResourceAddressString, item])),
            withdrawMethodsAreAccessible
        }
    }

    private async areAccountsWithdrawMethodsAccessible(accountAddress: ComponentAddressString): Promise<boolean> {
        const flashyfiAddresses = await this.getFlashyfiAddresses()
        const accessRuleChain = await this.getAccountAccessRuleChain(accountAddress)

        for (const methodName of ["withdraw_by_amount", "withdraw_by_ids"]) {
            const methodAuths: Array<any> = accessRuleChain["method_auth"]
            const methodAuth = methodAuths.find(methodAuth => {
                return methodAuth["method"]["name"] === methodName
            })
            if (!methodAuth) {
                return false
            }

            // Very simple heuristic that checks that there is a rule that allows the Flashyfi badge to access this method
            // This is of course by no means bulletproof and only design to guard against low level adversarial behavior
            const authString = JSON.stringify(methodAuth)
            const accessible = authString.includes(flashyfiAddresses.flashyfiBadgeResource) && authString.includes("AnyOf")
            if (!accessible) {
                return false
            }
        }

        return true
    }

    private parseFeeEnumNode(feeNode: any): [FeeType, number] {
        const discriminator = feeNode.variant.discriminator
        if (discriminator === "0") {
            return [FeeType.PERCENTAGE, parseFloat(feeNode.fields[0].value)]
        } else if (discriminator === "1") {
            return [FeeType.FIXED, parseFloat(feeNode.fields[0].elements[0].value)]
        } else {
            throw Error("Unhandled fee enum discriminator: " + discriminator)
        }
    }

    private async getAccountPublicKeyGlobalId(account: ComponentAddressString): Promise<{ resourceAddress: string, localId: string }> {
        const accessRuleChain = await this.getAccountAccessRuleChain(account)
        // @ts-ignore
        const nonFungible = accessRuleChain.default_auth.access_rule.proof_rule.resource
        return {
            resourceAddress: nonFungible.resource_address,
            localId: nonFungible.non_fungible_id.simple_rep
        }
    }

    private async getAccountAccessRuleChain(account: ComponentAddressString): Promise<any> {
        const response = await this.stateApi.entityDetails({
            entityDetailsRequest: {
                address: account
            }
        })
        const details = response.details!! as EntityDetailsResponseComponentDetails;
        return (details.access_rules_chain as Array<object>)[1]
    }

    async decodedSborData(sborHexData: string): Promise<any> {
        const ret = await this.radixEngineToolkitPromise
        return ret.sborDecode(sborHexData, NETWORK_ID_BETANET)
    }
}

export type AccountResources = {
    fungibles: Array<FungibleResourcesCollectionItem>,
    nonFungibles: Array<NonFungibleResourcesCollectionItem>
}

export default new FlashyfiRepo()