import {createContext, Component} from "react"
import Sdk, {ManifestBuilder} from "@radixdlt/alphanet-walletextension-sdk"
import { StateApi, TransactionApi } from '@radixdlt/alphanet-gateway-api-v0-sdk'

const BlockContext = createContext(null)

class BlockProvider extends Component {
    constructor(props) {
        super(props)

        this.sdk = Sdk()
        this.transactionApi = new TransactionApi()
        this.stateApi = new StateApi()
        this.componentAddress = "component_tdx_a_1qfjxhdlla3w5hp6pgyjypuwuuuga4hh8sw6gutvyuc4sxentdx"

        this.state = {
            address: null
        }
    }

    connect = async () => {
        const result = await this.sdk.request({
            accountAddresses: {},
        })

        if (result.isErr()) {
            console.log(result.error)
            return
        }

        const { accountAddresses } = result.value
        this.setState({address: (accountAddresses) ? accountAddresses[0].address : null})
    }

    buy = async () => {
        let manifest = new ManifestBuilder()
            .callMethod(this.state.address, "lock_fee", ['Decimal("100")'])
            .withdrawFromAccountByAmount(this.state.address, 10, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
            .takeFromWorktopByAmount(10, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9", "bucket1")
            .callMethod(this.componentAddress, "buy_gumball", ['Bucket("bucket1")'])
            .callMethod(this.state.address, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
            .build()
            .toString();

        // Send manifest to extension for signing
        const hash = await this.sdk
            .sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) {
            console.log(hash.error)
        }

        // Fetch the receipt from the Gateway SDK
        const receipt = await this.transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: { intent_hash: hash.value },
        })
    }

    render() {
        return (
            <BlockContext.Provider value={{
                address: this.state.address,
                connect: this.connect,
                buy: this.buy
            }}>
                {this.props.children}
            </BlockContext.Provider>
        )
    }
}

export {
    BlockContext,
    BlockProvider
}