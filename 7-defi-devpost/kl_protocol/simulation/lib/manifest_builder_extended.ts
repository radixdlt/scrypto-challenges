import { Bucket, ComponentAddressString, Decimal, Expression } from "@radixdlt/wallet-sdk";
import { ManifestBuilder } from "./manifest_builder.js";

class ManifestBuilderExt extends ManifestBuilder {

    bucketId: number = 0

    depositEntireWorktop(componentAddress: ComponentAddressString) {
        return this.callMethod(
            componentAddress,
            "deposit_batch",
            [
                Expression("ENTIRE_WORKTOP")
            ]
        )
    }

    depositeFromWorktop(componentAddress: ComponentAddressString, resourceAddress: `resource_${string}`) {

        this.bucketId++

        return this
            .takeFromWorktop(resourceAddress, `bucket_${this.bucketId}`)
            .callMethod(
                componentAddress,
                "deposit",
                [
                    Bucket(`bucket_${this.bucketId}`)
                ]
            )

    }

    depositeFromWorktopByAmount(componentAddress: ComponentAddressString, resourceAddress: `resource_${string}`, amount: number) {

        this.bucketId++

        return this
            .takeFromWorktopByAmount(amount, resourceAddress, `bucket_${this.bucketId}`)
            .callMethod(
                componentAddress,
                "deposit",
                [
                    Bucket(`bucket_${this.bucketId}`)
                ]
            )
    }

    withdrawToBucket(componentAddress: ComponentAddressString, resourceAddress: `resource_${string}`, bucketName: string) {
        return this
            .withdrawFromAccount(componentAddress, resourceAddress)
            .takeFromWorktop(resourceAddress, bucketName)

    }

    withdrawToBucketByAmount(componentAddress: ComponentAddressString, resourceAddress: `resource_${string}`, bucketName: string, amount: number) {
        return this
            .withdrawFromAccountByAmount(componentAddress, amount, resourceAddress)
            .takeFromWorktop(resourceAddress, bucketName)
    }

    lockFee(componentAddress: ComponentAddressString, amount: number) {

        return this.callMethod(
            componentAddress,
            "lock_fee",
            [
                Decimal(amount)
            ]
        )

    }

}



export { ManifestBuilderExt };
