
import { ManifestBuilder } from './manifest';

const expected_manifest_1 = `TAKE_FROM_WORKTOP ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335") Bucket("bucket1");
TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("1.2") ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335") Bucket("bucket2");
TAKE_FROM_WORKTOP_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("dead"), NonFungibleId("beef")) ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335") Bucket("bucket3");
RETURN_TO_WORKTOP Bucket("bucket3");
ASSERT_WORKTOP_CONTAINS ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335");
ASSERT_WORKTOP_CONTAINS_BY_AMOUNT Decimal("3.4") ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335");
ASSERT_WORKTOP_CONTAINS_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("dead"), NonFungibleId("beef")) ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335");
POP_FROM_AUTH_ZONE Proof("proof1");
PUSH_TO_AUTH_ZONE Proof("proof1");
CLEAR_AUTH_ZONE;
CREATE_PROOF_FROM_AUTH_ZONE ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335") Proof("proof2");
CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT Decimal("1.2") ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335") Proof("proof3");
CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS TreeSet<NonFungibleId>(NonFungibleId("dead"), NonFungibleId("beef")) ResourceAddress("034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335") Proof("proof4");
CREATE_PROOF_FROM_BUCKET Bucket("bucket1") Proof("proof5");
CLONE_PROOF Proof("proof5") Proof("proof6");
DROP_PROOF Proof("proof6");
CALL_FUNCTION PackageAddress("01afd33a0aa465673a3ba9dc82444029620138a04f537d54f4cad8") "GumballMachine" "new" Decimal("1.2") "GUM";
CALL_METHOD ComponentAddress("0276ef419fc25b4b8bfd14c65bda76d15d73372693d3d9240de390") "buy_gumball" Bucket("bucket2");
PUBLISH_PACKAGE Bytes("010203");`

const expected_manifest_2 = `CALL_METHOD ComponentAddress("020000000000000000000000000000000000000000000000000002") "free_xrd" ;
TAKE_FROM_WORKTOP ResourceAddress("030000000000000000000000000000000000000000000000000004") Bucket("xrd");
CALL_FUNCTION PackageAddress("010000000000000000000000000000000000000000000000000003") "Account" "new_with_resource" Enum("Protected", Enum("ProofRule", Enum("Require", Enum("StaticNonFungible", NonFungibleAddress("030000000000000000000000000000000000000000000000000005044083a64afb4b630ce7683674a6cdcebc7007aef7cb08f10b2cd491b6ce24ca1204f88bd2a2068e27591f1c5cfbd4fddf9a51f7b2360d784ee1e8fbec8f7476a6"))))) Bucket("xrd");`;

describe('Manifest builder tests', function () {
    it('Test basic instructions', function () {
        const manifest = new ManifestBuilder()
            .takeFromWorktop('034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335', 'bucket1')
            .takeFromWorktopByAmount(1.2, '034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335', 'bucket2')
            .takeFromWorktopByIds(['dead', 'beef'], '034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335', 'bucket3')
            .returnToWorktop('bucket3')
            .assertWorktopContains('034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335')
            .assertWorktopContainsByAmount(3.4, '034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335')
            .assertWorktopContainsByIds(['dead', 'beef'], '034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335')
            .popFromAuthZone('proof1')
            .pushToAuthZone('proof1')
            .clearAuthZone()
            .createProofFromAuthZone('034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335', 'proof2')
            .createProofFromAuthZoneByAmount(1.2, '034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335', 'proof3')
            .createProofFromAuthZoneByIds(['dead', 'beef'], '034812f861a8081a76f19a860f9312854826e8dbd791cf23deb335', 'proof4')
            .createProofFromBucket('bucket1', 'proof5')
            .cloneProof('proof5', 'proof6')
            .dropProof('proof6')
            .callFunction('01afd33a0aa465673a3ba9dc82444029620138a04f537d54f4cad8', 'GumballMachine', 'new', ['Decimal("1.2") "GUM"'])
            .callMethod('0276ef419fc25b4b8bfd14c65bda76d15d73372693d3d9240de390', 'buy_gumball', ['Bucket("bucket2")'])
            .publishPackage(new Uint8Array([1, 2, 3]))
            .build();

        expect(manifest.toString()).toBe(expected_manifest_1);
    })

    it('Test new account', function () {
        const manifest = new ManifestBuilder()
            .newAccount('044083a64afb4b630ce7683674a6cdcebc7007aef7cb08f10b2cd491b6ce24ca1204f88bd2a2068e27591f1c5cfbd4fddf9a51f7b2360d784ee1e8fbec8f7476a6')
            .build();

        expect(manifest.toString()).toBe(expected_manifest_2);
    })
})