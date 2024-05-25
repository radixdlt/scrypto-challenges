"""
This example shows how batch transfers from one account into multiple accounts can be performed. A
CSV file is used to keep a log of all of the transfers to perform. The file contains the destination
address, the address of the resource to send, and the amount of resources to send. This example
reads the CSV file, processes it into a transaction manifest, and then constructs a transaction from
it.
"""

from radix_engine_toolkit import *
from typing import Tuple
import secrets
import csv
from dotenv import load_dotenv

load_dotenv()  # This loads the environment variables from .env
import os
from mnemonic import Mnemonic
import bip32utils


class MockGatewayApiClient:
    @staticmethod
    def current_epoch() -> int:
        return 100

    @staticmethod
    def submit_transaction(transaction: NotarizedTransaction) -> None:
        _compiled_notarized_transaction = transaction.compile()


def random_nonce() -> int:
    """
    Generates a random secure random number between 0 and 0xFFFFFFFF (u32::MAX)
    """
    return secrets.randbelow(0xFFFFFFFF)


def seed_phrase_to_private_key() -> PrivateKey:
    seed_phrases = os.getenv("SEED_PHRASE")
    seed = Mnemonic.to_seed(seed_phrases)
    bip32_root_key = bip32utils.BIP32Key.fromEntropy(seed)
    private_key_hex = bip32_root_key.PrivateKey().hex()
    private_key_bytes: bytes = bytes.fromhex(private_key_hex)
    private_key: PrivateKey = PrivateKey.new_secp256k1(bytes=private_key_bytes)
    return private_key


def get_well_known(network_id: int) -> {}:
    if network_id == 0x01:
        return {
            "xrd": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
            "secp256k1_signature_virtual_badge": "resource_rdx1nfxxxxxxxxxxsecpsgxxxxxxxxx004638826440xxxxxxxxxsecpsg",
            "ed25519_signature_virtual_badge": "resource_rdx1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxxed25sg",
            "package_of_direct_caller_virtual_badge": "resource_rdx1nfxxxxxxxxxxpkcllrxxxxxxxxx003652646977xxxxxxxxxpkcllr",
            "global_caller_virtual_badge": "resource_rdx1nfxxxxxxxxxxglcllrxxxxxxxxx002350006550xxxxxxxxxglcllr",
            "system_transaction_badge": "resource_rdx1nfxxxxxxxxxxsystxnxxxxxxxxx002683325037xxxxxxxxxsystxn",
            "package_owner_badge": "resource_rdx1nfxxxxxxxxxxpkgwnrxxxxxxxxx002558553505xxxxxxxxxpkgwnr",
            "validator_owner_badge": "resource_rdx1nfxxxxxxxxxxvdrwnrxxxxxxxxx004365253834xxxxxxxxxvdrwnr",
            "account_owner_badge": "resource_rdx1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxxaccwnr",
            "identity_owner_badge": "resource_rdx1nfxxxxxxxxxxdntwnrxxxxxxxxx002876444928xxxxxxxxxdntwnr",
            "package_package": "package_rdx1pkgxxxxxxxxxpackgexxxxxxxxx000726633226xxxxxxxxxpackge",
            "resource_package": "package_rdx1pkgxxxxxxxxxresrcexxxxxxxxx000538436477xxxxxxxxxresrce",
            "account_package": "package_rdx1pkgxxxxxxxxxaccntxxxxxxxxxx000929625493xxxxxxxxxaccntx",
            "identity_package": "package_rdx1pkgxxxxxxxxxdntyxxxxxxxxxxx008560783089xxxxxxxxxdntyxx",
            "consensus_manager_package": "package_rdx1pkgxxxxxxxxxcnsmgrxxxxxxxxx000746305335xxxxxxxxxcnsmgr",
            "access_controller_package": "package_rdx1pkgxxxxxxxxxcntrlrxxxxxxxxx000648572295xxxxxxxxxcntrlr",
            "transaction_processor_package": "package_rdx1pkgxxxxxxxxxtxnpxrxxxxxxxxx002962227406xxxxxxxxxtxnpxr",
            "metadata_module_package": "package_rdx1pkgxxxxxxxxxmtdataxxxxxxxxx005246577269xxxxxxxxxmtdata",
            "royalty_module_package": "package_rdx1pkgxxxxxxxxxryaltyxxxxxxxxx003849573396xxxxxxxxxryalty",
            "role_assignment_module_package": "package_rdx1pkgxxxxxxxxxarulesxxxxxxxxx002304462983xxxxxxxxxarules",
            "genesis_helper_package": "package_rdx1pkgxxxxxxxxxgenssxxxxxxxxxx004372642773xxxxxxxxxgenssx",
            "faucet_package": "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet",
            "pool_package": "package_rdx1pkgxxxxxxxxxplxxxxxxxxxxxxx020379220524xxxxxxxxxplxxxx",
            "transaction_tracker_package": "package_rdx1pkgxxxxxxxxxtxtrakxxxxxxxxx000595975309xxxxxxxxxtxtrak",
            "consensus_manager": "consensusmanager_rdx1scxxxxxxxxxxcnsmgrxxxxxxxxx000999665565xxxxxxxxxcnsmgr",
            "genesis_helper": "component_rdx1cptxxxxxxxxxgenssxxxxxxxxxx000977302539xxxxxxxxxgenssx",
            "faucet": "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet",
            "transaction_tracker": "transactiontracker_rdx1stxxxxxxxxxxtxtrakxxxxxxxxx006844685494xxxxxxxxxtxtrak"
        }

    elif network_id == 0x02:
        return {
            "xrd": "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
            "secp256k1_signature_virtual_badge": "resource_tdx_2_1nfxxxxxxxxxxsecpsgxxxxxxxxx004638826440xxxxxxxxxcdcdpa",
            "ed25519_signature_virtual_badge": "resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa",
            "package_of_direct_caller_virtual_badge": "resource_tdx_2_1nfxxxxxxxxxxpkcllrxxxxxxxxx003652646977xxxxxxxxxfzcnwk",
            "global_caller_virtual_badge": "resource_tdx_2_1nfxxxxxxxxxxglcllrxxxxxxxxx002350006550xxxxxxxxxqtcnwk",
            "system_transaction_badge": "resource_tdx_2_1nfxxxxxxxxxxsystxnxxxxxxxxx002683325037xxxxxxxxxcss8hx",
            "package_owner_badge": "resource_tdx_2_1nfxxxxxxxxxxpkgwnrxxxxxxxxx002558553505xxxxxxxxxfzgzzk",
            "validator_owner_badge": "resource_tdx_2_1nfxxxxxxxxxxvdrwnrxxxxxxxxx004365253834xxxxxxxxxyerzzk",
            "account_owner_badge": "resource_tdx_2_1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxx4vczzk",
            "identity_owner_badge": "resource_tdx_2_1nfxxxxxxxxxxdntwnrxxxxxxxxx002876444928xxxxxxxxx98tzzk",
            "package_package": "package_tdx_2_1pkgxxxxxxxxxpackgexxxxxxxxx000726633226xxxxxxxxxehawfs",
            "resource_package": "package_tdx_2_1pkgxxxxxxxxxresrcexxxxxxxxx000538436477xxxxxxxxxmn4mes",
            "account_package": "package_tdx_2_1pkgxxxxxxxxxaccntxxxxxxxxxx000929625493xxxxxxxxx9jat20",
            "identity_package": "package_tdx_2_1pkgxxxxxxxxxdntyxxxxxxxxxxx008560783089xxxxxxxxx4ewu80",
            "consensus_manager_package": "package_tdx_2_1pkgxxxxxxxxxcnsmgrxxxxxxxxx000746305335xxxxxxxxxqe4rf2",
            "access_controller_package": "package_tdx_2_1pkgxxxxxxxxxcntrlrxxxxxxxxx000648572295xxxxxxxxxqewm72",
            "transaction_processor_package": "package_tdx_2_1pkgxxxxxxxxxtxnpxrxxxxxxxxx002962227406xxxxxxxxxnvke82",
            "metadata_module_package": "package_tdx_2_1pkgxxxxxxxxxmtdataxxxxxxxxx005246577269xxxxxxxxxrpg925",
            "royalty_module_package": "package_tdx_2_1pkgxxxxxxxxxryaltyxxxxxxxxx003849573396xxxxxxxxxmwc82d",
            "role_assignment_module_package": "package_tdx_2_1pkgxxxxxxxxxarulesxxxxxxxxx002304462983xxxxxxxxx9fe8ce",
            "genesis_helper_package": "package_tdx_2_1pkgxxxxxxxxxgenssxxxxxxxxxx004372642773xxxxxxxxxsnkg30",
            "faucet_package": "package_tdx_2_1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxx3heqcz",
            "pool_package": "package_tdx_2_1pkgxxxxxxxxxplxxxxxxxxxxxxx020379220524xxxxxxxxxe4r780",
            "consensus_manager": "consensusmanager_tdx_2_1scxxxxxxxxxxcnsmgrxxxxxxxxx000999665565xxxxxxxxxv6cg29",
            "genesis_helper": "component_tdx_2_1cptxxxxxxxxxgenssxxxxxxxxxx000977302539xxxxxxxxx9cs7tj",
            "faucet": "component_tdx_2_1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxyulkzl"
        }

    else:
        return None


def main() -> None:
    # The network ID to use for this example.
    network_id: int = 0x02
    well_known = get_well_known(network_id)
    xrd = well_known["xrd"]
    account_w_badge = Address(os.getenv("ACCOUNT_ADDY"))
    updater_badge_address = Address(os.getenv("UPDATER_BADGE_RADDY"))
    component_addy = Address(os.getenv("COMP_ADDY"))
    print(f"xrd_address = {xrd}")

    # The private key of the account that the funds will originate from (the sender) is known. A
    # `PrivateKey` object is created out of it and the virtual account component address is derived
    # from it.
    private_key: PrivateKey = seed_phrase_to_private_key()
    public_key: PublicKey = private_key.public_key()
    account_address: Address = Address.virtual_account_address_from_public_key(
        public_key, network_id
    )
    print(f"Account address: {account_address.as_str()}")

    manifest_string: str = f"""
CALL_METHOD
    Address("{xrd}")
    "lock_fee"
    Decimal("100")
;
CALL_METHOD
    Address("{account_w_badge}")
    "create_proof_of_non_fungibles"
    Address("{updater_badge_address}")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("#0#")
    )
;
CALL_METHOD
    Address("{component_addy}")
    "update_dbs_to_now"
;
CALL_METHOD
    Address("{account_w_badge}")
    "try_deposit_batch_or_refund"
    Expression("ENTIRE_WORKTOP")
    Enum<0u8>()
;
    """
    manifest: TransactionManifest = TransactionManifest(
        Instructions.from_string(manifest_string, network_id),
        []
    )
    manifest.statically_validate()
    print(f"Constructed manifest: {manifest.instructions().as_str()}")

    # Constructing the transaction
    current_epoch: int = MockGatewayApiClient.current_epoch()
    transaction: NotarizedTransaction = (
        TransactionBuilder()
        .header(
            TransactionHeader(
                network_id,
                current_epoch,
                current_epoch + 10,
                random_nonce(),
                public_key,
                True,
                0,
            )
        )
        .manifest(manifest)
        .message(Message.NONE())
        .notarize_with_private_key(private_key)
    )

    # Printing out the transaction ID and then submitting the transaction to the
    # network.
    transaction_id: TransactionHash = transaction.intent_hash()
    print(f"Transaction ID: {transaction_id.as_str()}")

    MockGatewayApiClient.submit_transaction(transaction)


if __name__ == "__main__":
    main()
