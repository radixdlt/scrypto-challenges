import { RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit';
import { RADINFO } from 'src/app/consts';

export type Rdt = ReturnType<typeof RadixDappToolkit>;

export class RadInsuranceComponentInfo {
  public badgeTypes!: BadgeUser;
  public policies!: RadInsurancePolicy[];
  public resourceAddress!: string;
}

export class PolicyInfo {
  public id!: number;
  public name!: string;
  public description!: string;
  public insuredContributionPercentate!: number;
  public insurerRewardPercentRate!: number;
  public totalInsurersAmount!: number;
  public totalInsuredsCoverAmount!: number;
  public serviceFees!: number; 
}

export class BadgeUser {
  public adminBadgeResourceAddress!: string;
  public insuredBadgeResourceAddress!: string;
  public insurerBadgeResourceAddress!: string;
  public insuredClaimBadgeResourceAddress!: string;
  public insurerMarketListBadgeResourceAddress!: string;
}

export class RadInsurancePolicy {
  public id!: number;
  public policyAdress!: string;
}

export class AccountFungibleInfo {
  constructor(fungibleAdress: string, amount: number, isXRD: boolean) {
    this.fungibleAdress = fungibleAdress;
    this.amount = amount;
    this.isXRD = isXRD;
  }
  public fungibleAdress: string;
  public amount: number;
  public isXRD?: boolean;
}

export class AccountNonFigibleInfo {
  public NonfungibleAdress!: string;
  public amount!: number;
}
export class CreatePolicy {
  public name!: string;
  public description!: string;
  public insurerRewardPercentRate!: number;
  public amount!: number;
}

export class InvestAsInsurerModel{
   public amountToInvest! : number;
}
