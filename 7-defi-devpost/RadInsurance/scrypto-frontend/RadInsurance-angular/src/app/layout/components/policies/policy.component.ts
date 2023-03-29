// import { Component } from '@angular/core';
// import { Account, RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit';
// import { PolicyManifestService } from 'src/app/services/manifest/PolicyManifestService';
// import { RadInsuranceComponentManifestService } from 'src/app/services/manifest/RadInsuranceComponentManifestService';
// import {RadinsuranceInfoService} from 'src/app/services/radinsurance-info.service';
// import {
//   BadgeUser,
//   RadInsuranceComponentInfo,
//   RadInsurancePolicy,
//   Rdt,
// } from 'src/app/types';
// import { ConnectionService } from '../connection-button/connection.service';


// @Component({
//   selector: 'app-policy',
//   templateUrl: './policy.component.html',
//   styleUrls: ['./policy.component.css'],
// })
// export class PolicyComponent {
//   private rdt!:Rdt; 
//   constructor(
//     private readonly policyManifestService: PolicyManifestService,
//     private readonly radInsuranceComponentManifestService: RadInsuranceComponentManifestService,
//     private readonly radinsuranceInfoService : RadinsuranceInfoService,
//     private readonly connectionService: ConnectionService,
//   ) {
//     setTimeout(() => {
//       this.rdt = this.connectionService.getRdt();
//     }, 500);
//   }

  
//   private radInsuranceComponentInfo = this.radinsuranceInfoService.getRadInsuranceComponentInfo(""); 

//   public async createPolicy() {
//     let manifest = this.policyManifestService.getCreatePolicyManifest("",
//       'Assurance auto',
//       'Ceci est un test',
//       2,
//       2,
//       (await this.radInsuranceComponentInfo).badgeTypes
//     );
//     this.invokeManifest(manifest);
//   }

//   public async instanciateRadInsurance(accountAdress: string) {
//      this.connectionService.getAccount().then(async (o) => {
//       this.radInsuranceComponentManifestService.getRadInsuranceComponentManifest("");
//         accountAdress
//       );
//       this.invokeManifest(manifest);
//     }); 
    

//   }

//   public async subscribeToInsurance(
//     insuredAddress: string,
//     policyId: number,
//     amount: number
//   ) {
//     let manifest =
//       this.policyManifestService.getSubscribeToInsurancePolicyManifest(
//         insuredAddress,
//         policyId,
//         amount
//       );
//     this.invokeManifest(manifest);
//   }

//   public async investAsInsurer(
//     insurerAddress: string,
//     policyId: number,
//     amount: number
//   ) {
//     let manifest = this.policyManifestService.getInvestAsInsurerManifest(
//       insurerAddress,
//       policyId,
//       amount
//     );
//     this.invokeManifest(manifest);
//   }

//   private async invokeManifest(manifest: string) {
//     console.log('init component manifest : ', manifest);

//     const result = await this.rdt.sendTransaction({
//       transactionManifest: manifest,
//       version: 1,
//     });

//     if (result.isErr()) {
//       console.log('transaction id : ', result.error);
//       throw result.error;
//     }

//     console.log('Success : ', result);
//   }
// }
