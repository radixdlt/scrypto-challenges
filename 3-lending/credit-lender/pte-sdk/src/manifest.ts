import { Buffer } from 'buffer';

export class Manifest {
    instructions: string[];

    constructor(instructions: string[]) {
        this.instructions = instructions;
    }

    public toString(): string {
        return this.instructions.join('\n');
    }
}

export class ManifestBuilder {
    private readonly instructions: string[];
    private readonly buckets: Map<string, number>;
    private readonly proofs: Map<string, number>;
    private id_allocator: number;

    constructor() {
        this.instructions = [];
        this.buckets = new Map<string, number>();
        this.proofs = new Map<string, number>();
        this.id_allocator = 512;
    }

    /**
     * Take all the given resource from worktop.
     * 
     * @param resourceAddress The resource address
     * @param bucketName The name of the new bucket
     * @returns 
     */
    takeFromWorktop(resourceAddress: string, bucketName: string): ManifestBuilder {
        this.instructions.push('TAKE_FROM_WORKTOP ResourceAddress("' + resourceAddress + '") Bucket("' + bucketName + '");')
        this.buckets.set(bucketName, this.id_allocator++);
        return this;
    }

    /**
     * Take some amount of resource from worktop.
     * 
     * @param amount The amount
     * @param resourceAddress The resource address
     * @param bucketName The name of the new bucket
     * @returns 
     */
    takeFromWorktopByAmount(amount: number, resourceAddress: string, bucketName: string): ManifestBuilder {
        this.instructions.push('TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("' + amount + '") ResourceAddress("' + resourceAddress + '") Bucket("' + bucketName + '");')
        this.buckets.set(bucketName, this.id_allocator++);
        return this;
    }

    /**
     * Take some non-fungibles from worktop.
     * 
     * @param nonFungibleIds The non-fungible IDs
     * @param resourceAddress The resource address
     * @param bucketName The name of the new bucket
     * @returns 
     */
    takeFromWorktopByIds(nonFungibleIds: string[], resourceAddress: string, bucketName: string): ManifestBuilder {
        this.instructions.push('TAKE_FROM_WORKTOP_BY_IDS ' + this.formatNonFungibleIds(nonFungibleIds) + ' ResourceAddress("' + resourceAddress + '") Bucket("' + bucketName + '");')
        this.buckets.set(bucketName, this.id_allocator++);
        return this;
    }

    /**
     * Returns a bucket to worktop.
     * 
     * @param bucketName The bucket name
     * @returns
     */
    returnToWorktop(bucketName: string) {
        this.instructions.push('RETURN_TO_WORKTOP Bucket("' + bucketName + '");')
        return this;
    }

    /**
     * Asserts worktop contains resource.
     * 
     * @param resourceAddress The resource address
     * @returns
     */
    assertWorktopContains(resourceAddress: string): ManifestBuilder {
        this.instructions.push('ASSERT_WORKTOP_CONTAINS ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Asserts worktop contains some amount of resource.
     * 
     * @param amount The amount
     * @param resourceAddress The resource address
     * @returns
     */
    assertWorktopContainsByAmount(amount: number, resourceAddress: string): ManifestBuilder {
        this.instructions.push('ASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal("' + amount + '") ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Asserts worktop contains some non-fungibles.
     * 
     * @param nonFungibleIds The non-fungible IDs
     * @param resourceAddress The resource address
     * @returns
     */
    assertWorktopContainsByIds(nonFungibleIds: string[], resourceAddress: string): ManifestBuilder {
        this.instructions.push('ASSERT_WORKTOP_CONTAINS_BY_IDS ' + this.formatNonFungibleIds(nonFungibleIds) + ' ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Pops the most recent proof from the auth zone.
     * 
     * @param proofName The name of the new proof
     * @returns 
     */
    popFromAuthZone(proofName: string): ManifestBuilder {
        this.instructions.push('POP_FROM_AUTH_ZONE Proof("' + proofName + '");')
        this.proofs.set(proofName, this.id_allocator++);
        return this;
    }

    /**
     * Pushes a proof onto the auth zone.
     * 
     * @param proofName The proof name
     * @returns 
     */
    pushToAuthZone(proofName: string): ManifestBuilder {
        this.instructions.push('PUSH_TO_AUTH_ZONE Proof("' + proofName + '");')
        return this;
    }

    /**
     * Clears the auth zone.
     * 
     * @returns 
     */
    clearAuthZone(): ManifestBuilder {
        this.instructions.push('CLEAR_AUTH_ZONE;');
        return this;
    }

    /**
     * Creates a composite proof from the auth zone with all the given resource.
     * 
     * @param resourceAddress The resource address
     * @param proofName The name of the new proof
     * @returns 
     */
    createProofFromAuthZone(resourceAddress: string, proofName: string): ManifestBuilder {
        this.instructions.push('CREATE_PROOF_FROM_AUTH_ZONE ResourceAddress("' + resourceAddress + '") Proof("' + proofName + '");')
        this.proofs.set(proofName, this.id_allocator++);
        return this;
    }

    /**
     * Creates a composite proof from the auth zone for the given amount.
     * 
     * @param amount The amount
     * @param resourceAddress The resource address
     * @param proofName The name of the new proof
     * @returns 
     */
    createProofFromAuthZoneByAmount(amount: number, resourceAddress: string, proofName: string): ManifestBuilder {
        this.instructions.push('CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT Decimal("' + amount + '") ResourceAddress("' + resourceAddress + '") Proof("' + proofName + '");')
        this.proofs.set(proofName, this.id_allocator++);
        return this;
    }

    /**
      * Creates a composite proof from the auth zone for the give non-fungibles.
      * 
      * @param nonFungibleIds The non-fungible IDs
      * @param resourceAddress The resource address
      * @param proofName The name of the new proof
      * @returns 
      */
    createProofFromAuthZoneByIds(nonFungibleIds: string[], resourceAddress: string, proofName: string): ManifestBuilder {
        this.instructions.push('CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS ' + this.formatNonFungibleIds(nonFungibleIds) + ' ResourceAddress("' + resourceAddress + '") Proof("' + proofName + '");')
        this.proofs.set(proofName, this.id_allocator++);
        return this;
    }

    /**
     * Creates a composite proof from the auth zone for a given amount.
     * 
     * @param amount The amount
     * @param resourceAddress The resource address
     * @param proofName The name of the new proof
     * @returns 
     */
    createProofFromBucket(bucketName: string, proofName: string): ManifestBuilder {
        this.instructions.push('CREATE_PROOF_FROM_BUCKET Bucket("' + bucketName + '") Proof("' + proofName + '");')
        this.proofs.set(proofName, this.id_allocator++);
        return this;
    }

    /**
     * Clones a proof.
     * 
     * @param proofName The proof name
     * @param clone The clone proof name
     * @returns 
     */
    cloneProof(proofName: string, cloneName: string): ManifestBuilder {
        this.instructions.push('CLONE_PROOF Proof("' + proofName + '") Proof("' + cloneName + '");')
        this.proofs.set(cloneName, this.id_allocator++);
        return this;
    }

    /**
     * Drops a proof.
     * 
     * @param proofName The proof name
     * @returns 
     */
    dropProof(proofName: string): ManifestBuilder {
        this.instructions.push('DROP_PROOF Proof("' + proofName + '");')
        return this;
    }

    /**
     * Calls a function on a blueprint.
     * 
     * @param packageAddress  The package address
     * @param blueprintName  The blueprint name
     * @param functionName  The function name
     * @param args The arguments, which must be in manifest format, e.g. `1u8`, `"string"`, `Bucket("name")`
     */
    callFunction(packageAddress: string, blueprintName: string, functionName: string, args: string[]): ManifestBuilder {
        this.instructions.push('CALL_FUNCTION PackageAddress("' + packageAddress + '") "' + blueprintName + '" "' + functionName + '" ' + args.join(" ") + ';')
        return this;
    }

    /**
     * Calls a method on a component.
     * 
     * @param componentAddress  The component address
     * @param methodName The method name
     * @param args The arguments, which must be in manifest format, e.g. `1u8`, `"string"`, `Bucket("name")`
     * @returns 
     */
    callMethod(componentAddress: string, methodName: string, args: string[]): ManifestBuilder {
        this.instructions.push('CALL_METHOD ComponentAddress("' + componentAddress + '") "' + methodName + '" ' + args.join(" ") + ';')
        return this;
    }

    /**
     * Calls a method on a component with all resources on or off worktop.
     * 
     * @param componentAddress  The component address
     * @param methodName The method name
     * @returns 
     */
    callMethodWithAllResources(componentAddress: string, methodName: string): ManifestBuilder {
        this.instructions.push('CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + componentAddress + '") "' + methodName + '";');
        return this;
    }

    /**
     * Publishes a package.
     * @param code The package wasm code
     */
    publishPackage(code: Uint8Array): ManifestBuilder {
        var hex = Buffer.from(code).toString('hex');
        this.instructions.push('PUBLISH_PACKAGE Bytes("' + hex + '");');
        return this;
    }

    /**
   * Withdraws all the given resource from account.
   * 
   * @param accountAddress The account component address
   * @param resourceAddress The resource address
   * @param bucketName The name of the new bucket
   * @returns 
   */
    withdrawFromAccount(accountAddress: String, resourceAddress: string): ManifestBuilder {
        this.instructions.push('CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw" ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Withdraws some amount of resource from account.
     * 
     * @param accountAddress The account component address
     * @param amount The amount
     * @param resourceAddress The resource address
     * @returns 
     */
    withdrawFromAccountByAmount(accountAddress: String, amount: number, resourceAddress: string): ManifestBuilder {
        this.instructions.push('CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("' + amount + '") ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Withdraws some non-fungibles from account.
     * 
     * @param accountAddress The account component address
     * @param nonFungibleIds The non-fungible IDs
     * @param resourceAddress The resource address
     * @returns 
     */
    withdrawFromAccountByIds(accountAddress: String, nonFungibleIds: string[], resourceAddress: string): ManifestBuilder {
        this.instructions.push('CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_ids" ' + this.formatNonFungibleIds(nonFungibleIds) + ' ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
    * Creates proof of all the given resource from account.
    * 
    * @param accountAddress The account component address
    * @param resourceAddress The resource address
    * @param bucketName The name of the new bucket
    * @returns 
    */
    createProofFromAccount(accountAddress: String, resourceAddress: string): ManifestBuilder {
        this.instructions.push('CALL_METHOD ComponentAddress("' + accountAddress + '") "create_proof" ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Creates proof of some amount of resource from account.
     * 
     * @param accountAddress The account component address
     * @param amount The amount
     * @param resourceAddress The resource address
     * @returns 
     */
    createProofFromAccountByAmount(accountAddress: String, amount: number, resourceAddress: string): ManifestBuilder {
        this.instructions.push('CALL_METHOD ComponentAddress("' + accountAddress + '") "create_proof_by_amount" Decimal("' + amount + '") ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Creates proof of some non-fungibles from account.
     * 
     * @param accountAddress The account component address
     * @param nonFungibleIds The non-fungible IDs
     * @param resourceAddress The resource address
     * @returns 
     */
    createProofFromAccountByIds(accountAddress: String, nonFungibleIds: string[], resourceAddress: string): ManifestBuilder {
        this.instructions.push('CALL_METHOD ComponentAddress("' + accountAddress + '") "create_proof_by_ids" ' + this.formatNonFungibleIds(nonFungibleIds) + ' ResourceAddress("' + resourceAddress + '");')
        return this;
    }

    /**
     * Creates a new account.
     * @param publicKey The public key 
     * @returns 
     */
    newAccount(publicKey: String): ManifestBuilder {
        const auth = 'Enum("Protected", Enum("ProofRule", Enum("Require", Enum("StaticNonFungible", NonFungibleAddress("030000000000000000000000000000000000000000000000000005' + publicKey + '")))))';

        return this.callMethod('020000000000000000000000000000000000000000000000000002', 'free_xrd', [])
            .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
            .callFunction('010000000000000000000000000000000000000000000000000003', 'Account', 'new_with_resource', [auth, 'Bucket("xrd")']);
    }

    /**
     * Builds a transaction manifest.
     * 
     * @returns a transaction manifest
     */
    build(): Manifest {
        return new Manifest(this.instructions);
    }

    private formatNonFungibleIds(nonFungibleIds: string[]) {
        let ids = nonFungibleIds.map(id => 'NonFungibleId("' + id + '")').join(', ');
        return 'TreeSet<NonFungibleId>(' + ids + ')';
    }
}