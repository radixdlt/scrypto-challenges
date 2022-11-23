import Sdk, {ManifestBuilder} from '@radixdlt/alphanet-walletextension-sdk';
import {StateApi, TransactionApi, type TransactionReceipt} from '@radixdlt/alphanet-gateway-api-v0-sdk';
import {error} from "@sveltejs/kit";
import balance from "./balance_store";
import {parseAddress} from "./utils";
import {DaoSystemAddresses, RadSenseAddresses} from "./model";

const XRD_TOKEN_RESOURCE = "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9";
const RAD_SENSE_PACKAGE = "package_tdx_a_1q95sngwdc4sv7a48a7ftgskspw8k87lz7xrms4pk7dusr0unll";

class RadSenseRepo {
    sdk;
    transactionApi: TransactionApi;
    stateApi: StateApi;
    mocksBaseUrl: string;
    packageAddress: string = RAD_SENSE_PACKAGE;

    constructor(protocol: string, host: string) {
        this.sdk = Sdk();
        this.transactionApi = new TransactionApi();
        this.stateApi = new StateApi();
        this.mocksBaseUrl = protocol + "//" + host + "/mocks";
    }

    async instantiateRadSense(accountAddress: string): Promise<{
        radSenseComponent: string,
        rsa: RadSenseAddresses,
        arbitrationDaoAddresses: DaoSystemAddresses
    }> {
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "lock_fee", ['Decimal("20")'])
            .callFunction(this.packageAddress, "RadSense", "instantiate_global", [
                `Vec<Tuple>(Tuple("Mr. Nobody", ComponentAddress("${accountAddress}")))`,
                `Set<String>("${this.mocksBaseUrl}/tracking_api/Arbitrator")`,
                `Set<ResourceAddress>()`
            ])
            .build()
            .toString();

        const receipt = await this.submitTransaction(manifest);
        const output1Json = receipt.output![1].data_json;
        const radSenseComponent = parseAddress(output1Json.elements[0]);
        const radSenseAddresses = new RadSenseAddresses(output1Json.elements[1]);
        const arbitrationDaoAddresses = new DaoSystemAddresses(output1Json.elements[2]);
        return {
            radSenseComponent,
            rsa: radSenseAddresses,
            arbitrationDaoAddresses
        };
    }

    async registerAdBroker(accountAddress: string, radSenseComponent: string): Promise<string> {
        const registerUserRequest = `Enum("User", Enum("AdBroker", Struct("${this.mocksBaseUrl}/broker_api", "${this.mocksBaseUrl}/tracking_api/AdBroker", Decimal("0.1"))), None)`;
        return this.registerUser(accountAddress, radSenseComponent, registerUserRequest)
    }

    async registerAdvertiser(accountAddress: string, radSenseComponent: string): Promise<string> {
        const registerUserRequest = `Enum("User", Enum("Advertiser", Struct(Some("${this.mocksBaseUrl}/tracking_api/Advertiser"))), None)`;
        return this.registerUser(accountAddress, radSenseComponent, registerUserRequest)
    }

    async registerAdSlotProvider(accountAddress: string, radSenseComponent: string): Promise<string> {
        const registerUserRequest = `Enum("User", Enum("AdSlotProvider", Struct(Some("${this.mocksBaseUrl}/tracking_api/AdSlotProvider"))), None)`;
        return this.registerUser(accountAddress, radSenseComponent, registerUserRequest)
    }

    private async registerUser(accountAddress: string, radSenseComponent: string, registerUserRequest: string): Promise<string> {
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "lock_fee", ['Decimal("20")'])
            .callMethod(radSenseComponent, "register", [registerUserRequest])
            .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
            .build()
            .toString();

        const receipt = await this.submitTransaction(manifest);
        const output1Json = receipt.output![1].data_json;
        return parseAddress(output1Json.elements[1]);
    }

    async registerAdSlot(accountAddress: string, radSenseComponent: string, rsa: RadSenseAddresses, ad_slot_provider_id: string,
                         approved_broker_user_id: string, width: number, height: number): Promise<string> {
        const badgeProofName = "ad_slot_provider_badge";
        const request = `Enum("AdSlot", Enum("Fixed", ${width}u16, ${height}u16), Vec<String>("finance", "news", "bitcoin"), Proof("${badgeProofName}"), Vec<NonFungibleId>(NonFungibleId("${approved_broker_user_id}")))`;
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "lock_fee", ['Decimal("20")'])
            .createProofFromAccountByIds(accountAddress, [ad_slot_provider_id], rsa.ad_slot_provider_resource)
            .popFromAuthZone(`${badgeProofName}`)
            .callMethod(radSenseComponent, "register", [request])
            .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
            .build()
            .toString();
        manifest = manifest.replace("TreeSet", "Set");

        const receipt = await this.submitTransaction(manifest);
        const output1Json = receipt.output![3].data_json;
        return parseAddress(output1Json.elements[1]);
    }

    async registerAd(accountAddress: string, radSenseComponent: string, rsa: RadSenseAddresses, advertiserId: string,
                     brokerUserId: string, imageUrl: string, linkUrl: string, hoverText: string, costPerClick: number): Promise<string> {
        const adBudgetAmount = costPerClick * 10;
        const badgeProofName = "advertiser_badge";
        const adBudgetPaymentBucketName = "ad_budget_payment";
        const request = `Enum("Ad",  Enum("Image", "${imageUrl}"), "${linkUrl}", "${hoverText}", Decimal("${costPerClick}"), Vec<String>("finance", "defi", "exchange"), Enum("Fixed", 128u16, 128u16), Proof("${badgeProofName}"), NonFungibleId("${brokerUserId}"), Decimal("10"), Bucket("${adBudgetPaymentBucketName}"))`;
        let manifest = new ManifestBuilder()
            .callMethod(accountAddress, "lock_fee", ['Decimal("20")'])
            .createProofFromAccountByIds(accountAddress, [advertiserId], rsa.advertiser_resource)
            .popFromAuthZone(`${badgeProofName}`)
            .withdrawFromAccountByAmount(accountAddress, adBudgetAmount, XRD_TOKEN_RESOURCE)
            .takeFromWorktopByAmount(adBudgetAmount, XRD_TOKEN_RESOURCE, adBudgetPaymentBucketName)
            .callMethod(radSenseComponent, "register", [request])
            .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
            .build()
            .toString();
        manifest = manifest.replace("TreeSet", "Set");

        const receipt = await this.submitTransaction(manifest);
        const output1Json = receipt.output![5].data_json;
        return parseAddress(output1Json.elements[1]);
    }

    async submitTransaction(manifest: string): Promise<TransactionReceipt> {
        console.log(manifest)
        const hash = await this.sdk
            .sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) {
            throw hash.error
        }
        const receipt = await this.transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        const committedReceipt = receipt.committed.receipt;
        console.log(JSON.stringify(committedReceipt, null, 2));
        if (committedReceipt.status != "Succeeded") {
            alert("Transaction failed: " + committedReceipt.error_message);
            throw error(400, "Transaction failed: " + JSON.stringify(committedReceipt));
        }
        await balance.onBalanceChanged()

        return committedReceipt
    }
}

export default RadSenseRepo;
