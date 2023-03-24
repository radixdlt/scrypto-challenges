import { Injectable } from '@angular/core';
import { U128 } from '@radixdlt/radix-dapp-toolkit';
import { RADINFO } from 'src/app/consts';
import { BadgeUser, CreatePolicy } from '../../types';

@Injectable({
  providedIn: 'root',
})
export class PolicyManifestService {
  constructor() {}

  getRadInsuranceComponentAddress(): string {
    return (
      localStorage.getItem(
        RADINFO.RAD_INSURANCE_COMPONENT_LOCAL_STORAGE_NAME
      ) ?? RADINFO.RAD_INSURANCE_COMPONENT_ADDRESS
    );
  }

  getCreatePolicyManifest(
    adminAddress: string,
    createPolicy: CreatePolicy,
    badgeUser: BadgeUser
  ): string {
    let manifest = `   
            CALL_METHOD ComponentAddress("${adminAddress}") "withdraw_by_amount" Decimal("${
      createPolicy.amount
    }") ResourceAddress("${RADINFO.XRD_RESOURCE_ADDRESS}");      
            CALL_METHOD ComponentAddress("${adminAddress}") "create_proof" ResourceAddress("${
      badgeUser.adminBadgeResourceAddress
    }");      
            TAKE_FROM_WORKTOP ResourceAddress("${
              RADINFO.XRD_RESOURCE_ADDRESS
            }") Bucket("initial_liquidity");
            CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "create_policy" "${
      createPolicy.name
    }" "${createPolicy.description}" Decimal("${
      createPolicy.insurerRewardPercentRate
    }") Bucket("initial_liquidity");
        `;

    return manifest;
  }

  getSubscribeToInsurancePolicyManifest(
    insuredAddress: string,
    policyId: number,
    amount: number,
    serviceFess : number
  ): string {
    let withdrawAmount = amount + RADINFO.RAD_INSURANCE_FEE;
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
        CALL_METHOD ComponentAddress("${insuredAddress}") "withdraw_by_amount" Decimal("${withdrawAmount}") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }");      
        TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${
          serviceFess
        }") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("service_fee");
        TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${amount}") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("deposit_amount");
        CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "subscribe_to_insurance_policy" ${formattedPolicyId} Decimal("${amount}") Bucket("deposit_amount") Bucket("service_fee");      
        CALL_METHOD ComponentAddress("${insuredAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }

  getInvestAsInsurerManifest(
    insurerAddress: string,
    policyId: number,
    amount: number,
    serviceFees : number
  ): string {
    let withdrawAmount =  Number.parseFloat(amount.toString()) + Number.parseFloat(serviceFees.toString());
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
        CALL_METHOD ComponentAddress("${insurerAddress}") "withdraw_by_amount" Decimal("${withdrawAmount}") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }");      
        TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${
          serviceFees
        }") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("service_fee");
        TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${amount}") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("invest_amount");
        CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "invest_as_insurer" ${formattedPolicyId} Bucket("invest_amount") Bucket("service_fee");      
        CALL_METHOD ComponentAddress("${insurerAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }

  getRewardsWithdrawManifest(
    insurerAddress: string,
    policyId: number,
    badgeUser: BadgeUser
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
        CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" ResourceAddress("${
      badgeUser.insurerBadgeResourceAddress
    }");      
        POP_FROM_AUTH_ZONE Proof("insurer_proof");
        CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" ResourceAddress("${
      badgeUser.insurerBadgeResourceAddress
    }");      
        CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "rewards_withdrawal" ${formattedPolicyId} Proof("insurer_proof");            
        CALL_METHOD ComponentAddress("${insurerAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }

  getReportAClaimManifest(
    insuredAddress: string,
    policyId: number,
    claimReport: string,
    insuredBadgeProofId: string,
    claimAmount: number,
    badgeUser: BadgeUser
  ) {
    let today = new Date();
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
        CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS Array<NonFungibleLocalId>(NonFungibleLocalId(${insuredBadgeProofId})) ResourceAddress("${
      badgeUser.insuredBadgeResourceAddress
    }") Proof("insured_proof");
        POP_FROM_AUTH_ZONE Proof("insured_proof");
        CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS Array<NonFungibleLocalId>(NonFungibleLocalId(${insuredBadgeProofId})) ResourceAddress("${
      badgeUser.insuredBadgeResourceAddress
    }") Proof("insured_proof");
        CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "report_a_claim"  Proof("insured_proof")  ${formattedPolicyId} "${claimReport}" Decimal("${claimAmount}") ${
      today.toTimeString
    };                
        CALL_METHOD ComponentAddress("${insuredAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }

  getMakeClaimAsRefusedManifest(
    adminAddress: string,
    policyId: number,
    insuredClaimBadgeId: string,
    badgeUser: BadgeUser
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
      CALL_METHOD ComponentAddress("${adminAddress}") "create_proof" ResourceAddress("${
      badgeUser.adminBadgeResourceAddress
    }");                
      CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "make_claim_as_refused" ${formattedPolicyId} NonFungibleLocalId("${insuredClaimBadgeId}");
    `;

    return manifest;
  }

  getMakeClaimAsAcceptedManifest(
    adminAddress: string,
    policyId: number,
    insuredClaimBadgeId: string,
    badgeUser: BadgeUser
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
      CALL_METHOD ComponentAddress("${adminAddress}") "create_proof" ResourceAddress("${
      badgeUser.adminBadgeResourceAddress
    }");                
      CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "make_claim_as_accepted" ${formattedPolicyId} NonFungibleLocalId("${insuredClaimBadgeId}");
    `;

    return manifest;
  }

  getRewardsManifest(
    insurerAddress: string,
    policyId: number,
    badgeUser: BadgeUser
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
    CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" ResourceAddress("${
      badgeUser.insurerBadgeResourceAddress
    }");      
    POP_FROM_AUTH_ZONE Proof("insurer_proof");
    CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" ResourceAddress("${
      badgeUser.insurerBadgeResourceAddress
    }");
    CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "get_rewards" ${formattedPolicyId} Proof("insurer_proof");            
    `;
    return manifest;
  }

  getClaimWithdrawManifest(
    insuredAddress: string,
    policyId: number,
    badgeUser: BadgeUser
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
    CALL_METHOD ComponentAddress("${insuredAddress}") "create_proof" ResourceAddress("${
      badgeUser.insuredClaimBadgeResourceAddress
    }");      
    POP_FROM_AUTH_ZONE Proof("claim_proof");
    CALL_METHOD ComponentAddress("${insuredAddress}") "create_proof" ResourceAddress("${
      badgeUser.insuredClaimBadgeResourceAddress
    }");
    CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "claim_withdraw" ${formattedPolicyId} Proof("claim_proof"); 
    CALL_METHOD ComponentAddress("${insuredAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");           
    `;

    return manifest;
  }

  getListOnMarketplaceManifest(
    insurerAddress: string,
    policyId: number,
    listingAmount: number,
    badgeUser: BadgeUser,
    serviceFees : number
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
      CALL_METHOD ComponentAddress("${insurerAddress}") "withdraw_by_amount" Decimal("${
        serviceFees
    }") ResourceAddress("${RADINFO.XRD_RESOURCE_ADDRESS}");         
      CALL_METHOD ComponentAddress("${insurerAddress}") "withdraw_by_amount" Decimal("${listingAmount}") ResourceAddress("${
      badgeUser.insurerBadgeResourceAddress
    }");         
      TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${
        serviceFees
      }") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("service_fee");
      TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${listingAmount}") ResourceAddress("${
      badgeUser.insurerBadgeResourceAddress
    }") Bucket("insurer_bucket_to_list");
      CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "list_on_marketplace" ${formattedPolicyId}  Bucket("insurer_bucket_to_list") Bucket("service_fee") Decimal("${listingAmount}") ;
      CALL_METHOD ComponentAddress("${insurerAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }

  getDeListOnMarketplaceManifest(
    insurerAddress: string,
    policyId: number,
    badgeUser: BadgeUser,
    serviceFees : number
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let manifest = `
      CALL_METHOD ComponentAddress("${insurerAddress}") "withdraw_by_amount" Decimal("${
        serviceFees
    }") ResourceAddress("${RADINFO.XRD_RESOURCE_ADDRESS}");         
      CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" ResourceAddress("${
      badgeUser.insurerMarketListBadgeResourceAddress
    }");
      POP_FROM_AUTH_ZONE Proof("insurer_listing_proof");         
      TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${
        serviceFees
      }") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("service_fee");
      CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" ResourceAddress("${
      badgeUser.insurerMarketListBadgeResourceAddress
    }");
      CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "delist_on_marketplace" ${formattedPolicyId} Bucket("service_fee") Bucket("insurer_listing_proof");
      CALL_METHOD ComponentAddress("${insurerAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }

  getBuyOnMarketplaceManifest(
    policyId: number,
    buyerAddress: string,
    insurerListingBadgeId: string,
    paymentAmount: number,
    serviceFees: number
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let withdrawAmount = Number.parseFloat(paymentAmount.toString()) + Number.parseFloat(serviceFees.toString())
    let manifest = `
      CALL_METHOD ComponentAddress("${buyerAddress}") "withdraw_by_amount" Decimal("${withdrawAmount}") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }");         
      TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${
        serviceFees
      }") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("service_fee");
      TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("${paymentAmount}") ResourceAddress("${
      RADINFO.XRD_RESOURCE_ADDRESS
    }") Bucket("payment_amount");
      CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "buy_on_marketplace" ${formattedPolicyId} Bucket("payment_amount") Bucket("service_fee") NonFungibleLocalId(${insurerListingBadgeId});
      CALL_METHOD ComponentAddress("${buyerAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }

  getWithdrawalSaleAmountManifest(
    policyId: number,
    insurerAddress: string,
    insurerListingBadgeResourceAddress: number,
    paymentAmount: number
  ): string {
    let formattedPolicyId = U128(policyId.toString());
    let withdrawAmount = paymentAmount;
    let manifest = `
      CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" Decimal("${withdrawAmount}") ResourceAddress("${insurerListingBadgeResourceAddress}"); 
      POP_FROM_AUTH_ZONE Proof("insurer_listing_proof");     
      CALL_METHOD ComponentAddress("${insurerAddress}") "create_proof" Decimal("${withdrawAmount}") ResourceAddress("${insurerListingBadgeResourceAddress}"); 
      CALL_METHOD ComponentAddress("${this.getRadInsuranceComponentAddress()}") "withdrawal_sale_amount" ${formattedPolicyId} Proof("insurer_listing_proof");
      CALL_METHOD ComponentAddress("${insurerAddress}") "deposit_batch" Expression("ENTIRE_WORKTOP");
    `;

    return manifest;
  }
}
