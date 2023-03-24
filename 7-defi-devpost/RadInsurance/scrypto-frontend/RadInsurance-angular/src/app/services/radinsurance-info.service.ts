import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { RADINFO } from '../consts';
import {
  BadgeUser,
  RadInsuranceComponentInfo,
  RadInsurancePolicy,
} from '../types';

@Injectable({
  providedIn: 'root',
})
export class RadinsuranceInfoService {
  public adminBadgeResourceAddress!: string;
  public radInfo = RADINFO;
  private entityDetailsUrl = `${this.radInfo.BASE_BETANET_URL}/entity/details`;
  private radInsuranceComponentAddress : string = ""; 
  constructor(private http: HttpClient) {
    this.radInsuranceComponentAddress = localStorage.getItem(RADINFO.RAD_INSURANCE_COMPONENT_LOCAL_STORAGE_NAME) 
    ?? RADINFO.RAD_INSURANCE_COMPONENT_ADDRESS;
  }

  public getEntityDetails(address: string): Promise<any> {
    // possible de mettre cela en cache ?
    return new Promise((resolve, reject) => {
      this.http
        .post(this.entityDetailsUrl, {
          address: address,
        })
        .subscribe((resp) => resolve(resp));
    });
  }


  public async getComponentAddressFromHash(hash : string) : Promise<string>{
    
    let apiResult : any = await  new Promise((resolve, reject) => {
      this.http
        .post(`${this.radInfo.BASE_BETANET_URL}/transaction/committed-details`,{
                "transaction_identifier": {
                "type": "intent_hash",
                "value_hex": hash
                }
          })
        .subscribe((resp) => resolve(resp));
    });


    console.log(apiResult); 
    return apiResult.details.referenced_global_entities[0] as string; 
  }

  public async getProfilBadgeAdress(
    entityDetails: any = null
  ): Promise<BadgeUser> {
    const radInsuranceDetails =
      entityDetails ??
      (await this.getEntityDetails(
        this.radInsuranceComponentAddress
      ));
    let adminBadgeResourceAddress: string =
      radInsuranceDetails.details.state.data_json[4];
    const otherBadgeTypes = radInsuranceDetails.details.state.data_json[5];

    let badgeUser = new BadgeUser();
    badgeUser.adminBadgeResourceAddress = adminBadgeResourceAddress;

    otherBadgeTypes.forEach((badge: any) => {
      if (badge[0] == RADINFO.INSURED_BADGE_ID) {
        badgeUser.insuredBadgeResourceAddress = badge[1];
      } else if (badge[0] == RADINFO.INSURER_BADGE_ID) {
        badgeUser.insurerBadgeResourceAddress = badge[1];
      } else if (badge[0] == RADINFO.INSURED_CLAIM_BADGE_ID) {
        badgeUser.insuredClaimBadgeResourceAddress = badge[1];
      } else if (badge[0] == RADINFO.INSURER_MARKET_LISTING_BADGE_ID) {
        badgeUser.insurerMarketListBadgeResourceAddress = badge[1];
      }
    });
    return badgeUser;
  }
  public async getAdminBadgeResourceAddress(
    entityDetails: any = null
  ): Promise<string> {
    const radInsuranceDetails =
      entityDetails ??
      (await this.getEntityDetails(
        this.radInsuranceComponentAddress
      ));
    return radInsuranceDetails.details.state.data_json[4];
  }

  // ressource for radInsurance service
  public async getRessourceAddress(entityDetails: any = null): Promise<string> {
    const radInsuranceDetails =
      entityDetails ??
      (await this.getEntityDetails(
        this.radInsuranceComponentAddress
      ));
    return radInsuranceDetails.details.state.data_json[3];
  }

  public async getPolicies(
    entityDetails: any = null
  ): Promise<RadInsurancePolicy[]> {
    const radInsuranceDetails =
      entityDetails ??
      (await this.getEntityDetails(
        this.radInsuranceComponentAddress
      ));
    const policies: RadInsurancePolicy[] = [];
    radInsuranceDetails.details.state.data_json[0].forEach((value: any) => {
      const policy = new RadInsurancePolicy();
      (policy.id = value[0]), (policy.policyAdress = value[1]);
      policies.push(policy);
    });
    return policies;
  }
  // full info in RadInsurance
  public async getRadInsuranceComponentInfo(
  ): Promise<RadInsuranceComponentInfo> {
    const radInsuranceComponentInfo = new RadInsuranceComponentInfo();
    const radInsuranceDetails = await this.getEntityDetails(this.radInsuranceComponentAddress);

    const policies = await this.getPolicies(radInsuranceDetails);
    const resources = await this.getRessourceAddress(radInsuranceDetails);
    const badgeType = await this.getProfilBadgeAdress(radInsuranceDetails);

    radInsuranceComponentInfo.badgeTypes = badgeType;
    radInsuranceComponentInfo.policies = policies;
    radInsuranceComponentInfo.resourceAddress = resources;

    return radInsuranceComponentInfo;
  }
}
