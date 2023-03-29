import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Account } from '@radixdlt/radix-dapp-toolkit';
import { ConnectionService } from 'src/app/layout/components/connection-button/connection.service';
import { AccountService } from 'src/app/services/account.service';
import { RadinsuranceInfoService } from 'src/app/services/radinsurance-info.service';
import {
  InvestAsInsurerModel,
  PolicyInfo,
  RadInsuranceComponentInfo,
} from 'src/app/types';
import { PolicyService } from 'src/app/services/policy.service';
import { RADINFO } from 'src/app/consts';

@Component({
  selector: 'app-invest-as-insurer',
  templateUrl: './invest-as-insurer.component.html',
  styleUrls: ['./invest-as-insurer.component.css'],
})
export class InvestAsInsurerComponent {
  investAsInsurer!: InvestAsInsurerModel;
  radInsuranceComponentInfo!: RadInsuranceComponentInfo;
  policyInfo!: PolicyInfo;
  policyId!: number | null;
  accountAddress!: Account[];
  xrdAmount!: number;
  maxAmountToInvest!: number;
  displayAmountError: boolean = false;
  public isLoading: boolean = false;
  public showInfo: boolean = false;
  public showError: boolean = false;
  public showMessage: string | undefined = '';
  /**
   *
   */
  constructor(
    private radinsuranceInfoService: RadinsuranceInfoService,
    private policyServices: PolicyService,
    private accountService: AccountService,
    private connectionService: ConnectionService,
    private route: ActivatedRoute
  ) {
    this.investAsInsurer = new InvestAsInsurerModel();
  }
  public async ngOnInit() {
    this.radInsuranceComponentInfo =
      await this.radinsuranceInfoService.getRadInsuranceComponentInfo();
    this.policyId = this.route.snapshot.params['id'];
    let policy = this.radInsuranceComponentInfo.policies.find(
      (o) => o.id == this.policyId
    );
    if (policy)
      this.policyInfo = await this.policyServices.getPolicyInfo(policy);
    this.accountAddress = await this.connectionService.getAccount();
    let fungibles = await this.accountService.getFungiblesInfo(
      this.accountAddress[0].address
    );
    let xrdFungible = fungibles.find((f) => f.isXRD);
    if (xrdFungible) {
      this.maxAmountToInvest = Math.max(
        0,
        xrdFungible.amount - this.policyInfo.serviceFees - RADINFO.LOCK_FEE
      );
    } else {
      this.maxAmountToInvest = this.investAsInsurer.amountToInvest;
    }
  }
  public async onSubmit() {
    this.displayAmountError = false;
    if (
      this.maxAmountToInvest > 0 &&
      this.investAsInsurer.amountToInvest <= this.maxAmountToInvest
    ) {
      this.isLoading = true;
      this.showInfo = false;
      debugger;
      this.policyServices
        .invokeInvestAsInsurer(
          this.accountAddress[0].address,
          this.policyInfo.id,
          this.investAsInsurer.amountToInvest,
          this.policyInfo.serviceFees
        )
        .then((resp) => {
          debugger;
          if (resp) {
            this.isLoading = false;
            this.showInfo = true;
          }
          if (resp.isErr()) {
            this.showError = true;
            console.log('transaction id : ', resp.error);
            this.showMessage = resp.error.error;
          } else {
            console.log('Success : ', resp);
            this.showMessage = resp.value.transactionIntentHash;
          }
        });
    } else {
      this.displayAmountError = true;
    }
  }
}
