import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { RADINFO } from '../consts';
import { ConnectionService } from '../layout/components/connection-button/connection.service';
import {
  CreatePolicy,
  PolicyInfo,
  RadInsuranceComponentInfo,
  RadInsurancePolicy,
  Rdt,
} from '../types';
import { PolicyManifestService } from './manifest/PolicyManifestService';
import { RadinsuranceInfoService } from './radinsurance-info.service';

@Injectable({
  providedIn: 'root',
})
export class PolicyService {
  public radInfo = RADINFO;
  private entityDetailsUrl = `${this.radInfo.BASE_BETANET_URL}/entity/details`;
  private rdt!: Rdt;
  constructor(
    private http: HttpClient,
    private radinsuranceInfoService: RadinsuranceInfoService,
    private policyManifestService: PolicyManifestService,
    private connectionService: ConnectionService
  ) {
    setTimeout(() => {
      this.rdt = this.connectionService.getRdt();
    }, 500);
  }

  public async getPolicyInfo(
    policyAdress: RadInsurancePolicy
  ): Promise<PolicyInfo> {
    const policyDetails = await this.radinsuranceInfoService.getEntityDetails(
      policyAdress.policyAdress
    );

    const policyInfo = new PolicyInfo();
    policyInfo.id = policyAdress.id;
    policyInfo.name = policyDetails.details.state.data_json[1];
    policyInfo.description = policyDetails.details.state.data_json[2];
    policyInfo.insuredContributionPercentate =
      policyDetails.details.state.data_json[4];
    policyInfo.insurerRewardPercentRate =
      policyDetails.details.state.data_json[5];
    policyInfo.totalInsurersAmount = policyDetails.details.state.data_json[14];
    policyInfo.totalInsuredsCoverAmount =
      policyDetails.details.state.data_json[15];
    policyInfo.serviceFees = policyDetails.details.state.data_json[7];

    return policyInfo;
  }

  public async createPolicy(createPolicy: CreatePolicy) {
    const accountAddress = await this.connectionService.getAccount();
    const address = accountAddress[0].address;
    const radInfo =
      await this.radinsuranceInfoService.getRadInsuranceComponentInfo();
    return await this.invokeCreatePolicy(address, createPolicy, radInfo);
  }
  public async invokeCreatePolicy(
    accountAdress: string,
    createPolicy: CreatePolicy,
    radInfo: RadInsuranceComponentInfo
  ) {
    let manifest = this.policyManifestService.getCreatePolicyManifest(
      accountAdress,
      createPolicy,
      radInfo.badgeTypes
    );

    const result = await this.rdt.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });
    return result;
  }

  public async invokeInvestAsInsurer(accountAdress: string,
                                     policyId : number,
                                     amountToInvest : number,
                                     serviceFees : number) {
    
       let manifest = this.policyManifestService.getInvestAsInsurerManifest(accountAdress,
                                                                            policyId,amountToInvest,
                                                                            serviceFees);
       const result = await this.rdt.sendTransaction({
                                                                              transactionManifest: manifest,
                                                                              version: 1,
                                                      });
       return result;
   }
}
