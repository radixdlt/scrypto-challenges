import type {ComponentAddressString, ResourceAddressString} from "@radixdlt/radix-dapp-toolkit";
import type {FeeConfig, FlashyfiAccount, ResourceDetails} from "./types";
import {
    Bucket,
    ComponentAddress,
    Decimal,
    ManifestBuilder,
    ResourceAddress,
    Tuple
} from "@radixdlt/radix-dapp-toolkit";
import flashyfiRepo from "./flashyfiRepo";
import {FLASHYFI_COMPONENT_ADDRESS} from "./constants";
import {shortenAddress} from "./utils";


export function calculateLoan(resource: ResourceDetails, amount: number, lenderAccounts: Array<FlashyfiAccount>): LoanDefinition {
    if (!resource.fungible) {
        throw Error("Only fungible resources are supported at the moment")
    }

    type Lender = { account: FlashyfiAccount, feeConfig: FeeConfig, availableAmount: number }

    const lenders: Array<Lender> = lenderAccounts
        .map(account => {
            let feeConfig = account.fungibleFeeConfigs.find(config => config.resourceAddress === resource.address)!
            let borrowableAmount = account.availableFungibleResources.get(resource.address)!.amount.value
            return {account, feeConfig, availableAmount: parseFloat(borrowableAmount)}
        })
        // Only consider accounts that have resource available
        .filter(e => e.availableAmount > 0)
        // Sort by the fee we will have to pay. This is a bit naive because
        .sort((a, b) => {
            const aScore = a.feeConfig.feeValue! / a.availableAmount
            const bScore = b.feeConfig.feeValue! / b.availableAmount
            return aScore - bScore
        })

    let borrowInstructions: Array<BorrowInstruction> = []
    let openAmount = amount
    for (const lender of lenders) {
        let borrowAmount = Math.min(openAmount, lender.availableAmount)
        openAmount -= borrowAmount

        borrowInstructions.push({
            account: lender.account.accountAddress,
            amount: borrowAmount,
            fee: lender.feeConfig.feeValue!
        })

        if (openAmount <= 0) {
            break
        }
    }

    return new LoanDefinition(resource, borrowInstructions)
}

export type BorrowInstruction = {
    account: ComponentAddressString,
    amount: number
    fee: number
}

export class LoanDefinition {

    constructor(
        public readonly resource: ResourceDetails,
        public readonly borrowInstructions: Array<BorrowInstruction>
    ) {
    }

    get totalFee(): number {
        return this.borrowInstructions.map(instruction => instruction.fee).reduce((a, b) => a + b)
    }

    async generateManifest(borrowerAccount: ComponentAddressString): Promise<string> {
        const wellKnownAddresses = await flashyfiRepo.getWellKnownAddresses()
        const flashyfiAddresses = await flashyfiRepo.getFlashyfiAddresses()
        let manifest = new ManifestWithComments()
        let borrowResourceString = this.resource.getSymbolOrNameOrShortenedAddress()

        manifest.addComment("----- PART #1 - Take out the loan")
        manifest.addComment("----- Put each loan receipt into a bucket so we can return it")
        for (const [i, instruction] of this.borrowInstructions.entries()) {
            const loanReceiptBucketName = "loan_receipt_" + i
            manifest.addComment(`Borrow ${instruction.amount} ${borrowResourceString} from account ${shortenAddress(instruction.account)}`)
            manifest.addInstructions(builder => {
                builder.callMethod(
                    FLASHYFI_COMPONENT_ADDRESS,
                    "borrow",
                    [
                        `Enum(0u8, ${Tuple(ResourceAddress(this.resource.address), Decimal(instruction.amount))})`,
                        ComponentAddress(instruction.account)
                    ])
                    .takeFromWorktopByAmount(1, flashyfiAddresses.loanReceiptResource, loanReceiptBucketName)
            })
            manifest.addBlankLine()
        }

        manifest.addBlankLine()
        manifest.addComment("----- PART #2 - Use the loan, for example, to take advantage of an arbitrage opportunity")
        manifest.addComment("We simulate this here by calling the faucet component")
        manifest.addInstructions(builder => {
                builder.callMethod(wellKnownAddresses.faucet as ComponentAddressString, "free", [])
        })

        manifest.addBlankLine()
        manifest.addBlankLine()
        manifest.addComment("----- PART #3 - Repay the loan")

        for (const [i, instruction] of this.borrowInstructions.entries()) {
            const loanBucketName = "loan_" + i
            const feeBucketName = "fee_" + i
            const loanReceiptBucketName = "loan_receipt_" + i
            manifest.addComment(`Return ${instruction.amount} ${borrowResourceString} plus a fee of ${instruction.fee} XRD to account ${shortenAddress(instruction.account)}`)
            manifest.addInstructions(builder => {
                builder.takeFromWorktopByAmount(instruction.amount, this.resource.address, loanBucketName)
                builder.takeFromWorktopByAmount(instruction.fee, wellKnownAddresses.xrd as ResourceAddressString, feeBucketName)
                builder.callMethod(
                    FLASHYFI_COMPONENT_ADDRESS,
                    "repay_loan",
                    [
                        Bucket(loanReceiptBucketName),
                        Bucket(loanBucketName),
                        Bucket(feeBucketName),
                        ComponentAddress(instruction.account)
                    ])
            })
            manifest.addBlankLine()
        }

        manifest.addBlankLine()
        manifest.addComment("----- PART #4 - Profit")
        manifest.addComment("Deposit the profits (in this case the remaining funds from the faucet) into our account")
        manifest.addInstructions(builder => {
                builder.takeFromWorktop(wellKnownAddresses.xrd as ResourceAddressString, "profit")
                builder.callMethod(borrowerAccount, "deposit", [Bucket("profit")])
            }
        )

        return manifest.toString()
    }
}

class ManifestWithComments {
    manifest: string = ""

    addInstructions(block: (builder: ManifestBuilder) => void) {
        const builder = new ManifestBuilder()
        block(builder)
        this.manifest += builder.build().toString() + "\n"
    }

    addComment(text: string) {
        this.manifest += `# ${text} \n`
    }

    addBlankLine() {
        this.manifest += "\n"
    }

    toString() {
        return this.manifest
    }
}

