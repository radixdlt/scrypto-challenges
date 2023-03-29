import { Component } from '@angular/core';
import { PolicyService } from 'src/app/services/policy.service';
import { CreatePolicy, PolicyInfo } from 'src/app/types';

@Component({
  selector: 'app-create-policy',
  templateUrl: './create-policy.component.html',
  styleUrls: ['./create-policy.component.css'],
})
export class CreatePolicyComponent {
  policyInfo!: CreatePolicy;
  public isLoading: boolean = false;
  public showInfo: boolean = false;
  public showError: boolean = false;
  public showMessage: string | undefined = '';
  /**
   *
   */
  constructor(private policyService: PolicyService) {
    this.policyInfo = new CreatePolicy();
  }
  public ngOnInit() {}
  public onSubmit() {
    if (this.policyInfo) {
      this.isLoading = true;
      this.showInfo = false;
      this.policyService.createPolicy(this.policyInfo).then((resp) => {
        if (resp) {
          this.isLoading = false;
          this.showInfo = true;
        }
        if (resp.isErr()) {
          this.showError = true;
          this.showMessage = resp.error.error;
          console.log('transaction id : ', resp.error);
        } else {
          console.log('Success : ', resp);
          this.showMessage = resp.value.transactionIntentHash;
        }
      });
    }
  }
}
