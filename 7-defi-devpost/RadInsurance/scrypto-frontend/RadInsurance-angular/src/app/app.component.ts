import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { Observable } from 'rxjs';
import { __values } from 'tslib';
import { AppService } from './app.service';
import { RADINFO } from './consts';
import { AccountService } from './services/account.service';
import { RadinsuranceInfoService } from './services/radinsurance-info.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
})
export class AppComponent {
  title = 'RadInsurance-angular';
  public radInfo = RADINFO;
  /**
   *
   */
  ngOnInit() {
    const test = this.test();
    console.log('resultat : ', test);
  }
  constructor(
    private radinsuranceInfoService: RadinsuranceInfoService,
    private accountService: AccountService
  ) {}

  public test() {
    // this.radinsuranceInfoService.getProfilBadgeAdress().then((value) => {
    //   debugger;
    //   console.log(value);
    // });
    // this.accountService
    //   .getFungiblesInfo(this.radInfo.ACCOUNT_ADDRESS)
    //   .then((value) => {
    //     console.log(value);
    //   });
    // this.radinsuranceInfoService.getPolicies().then((value) => {
    //   console.log(value);
    // });
  }
  // public async getRadixComponentInfo(): Promise<RadInsuranceComponentInfo> {
  //   const data = await this.getDataRadixComponent();
  //   let policies: Map<number, string> = new Map<number, string>();
  //   data.details.state.data_json[0].forEach((policyIdAndAddressTab: any[]) => {
  //     policies.set(policyIdAndAddressTab[0], policyIdAndAddressTab[1]);
  //   });
  //   let adminBadgeResourceAddress: string = data.details.state.data_json[4];
  //   let resourceAddress: string = data.details.state.data_json[3];
  //   let badgeTypes: Map<number, string> = new Map<number, string>();
  //   data.details.state.data_json[5].forEach(
  //     (badgeTypeAndResourceAddress: any[]) => {
  //       badgeTypes.set(
  //         badgeTypeAndResourceAddress[0],
  //         badgeTypeAndResourceAddress[1]
  //       );
  //     }
  //   );
  //   let policiesInfos: PolicyInfo[] = [];
  //   for (let policyValue in policies) {
  //     let policyResponse = await this.getEntityDetails(policyValue);
  //     let policyInfo: PolicyInfo = new PolicyInfo(
  //       policyResponse.details.state.data_json[0],
  //       policyResponse.details.state.data_json[1],
  //       policyResponse.details.state.data_json[2],
  //       policyResponse.details.state.data_json[4],
  //       policyResponse.details.state.data_json[5],
  //       policyResponse.details.state.data_json[14],
  //       policyResponse.details.state.data_json[15]
  //     );
  //     policiesInfos.push(policyInfo);
  //   }
  //   return new RadInsuranceComponentInfo(
  //     badgeTypes,
  //     policiesInfos,
  //     resourceAddress
  //   );
  // }

  // private getEntityDetails(address: string): Promise<any> {
  //   return new Promise((resolve, reject) => {
  //     const result = this.http.post(
  //       'https://betanet.radixdlt.com/entity/details',
  //       address
  //     );
  //     resolve(result);
  //   });
  // }
  // private getDataRadixComponent(): Promise<any> {
  //   debugger;
  //   const test = new Promise((resolve, reject) => {
  //     this.appService.getRadixComponentInfo().subscribe((data) => {
  //       const t = '';
  //       resolve(data);
  //     });
  //   });
  //   return test;
  // }
}
