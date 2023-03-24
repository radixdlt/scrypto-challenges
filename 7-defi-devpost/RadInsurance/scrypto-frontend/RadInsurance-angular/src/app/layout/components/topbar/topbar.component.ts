import { Account, RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit';
import { Component, ViewChild } from '@angular/core';
import { ConnectionService } from '../connection-button/connection.service';
import { RADINFO } from 'src/app/consts';
import { AccountService } from 'src/app/services/account.service';
import { AccountFungibleInfo, Rdt } from 'src/app/types';
import { RadinsuranceInfoService } from 'src/app/services/radinsurance-info.service';
import { RadInsuranceComponentManifestService } from 'src/app/services/manifest/RadInsuranceComponentManifestService';
import { Router, ActivatedRoute, ParamMap } from '@angular/router';

@Component({
  selector: 'app-topbar',
  templateUrl: './topbar.component.html',
  styleUrls: ['./topbar.component.css'],
})
export class TopbarComponent {
  //@ViewChild('modal-example') dialog: any;
  public accountAddress!: Account[];
  public isAdmin: boolean = false;
  public showDialog: boolean = true;
  private radInsuranceComponentAddress!: string;
  private rdt!: Rdt;
  constructor(
    private connectionService: ConnectionService,
    private accountService: AccountService,
    private radinsuranceInfoService: RadinsuranceInfoService,
    private readonly radInsuranceComponentManifestService: RadInsuranceComponentManifestService,
    private route: ActivatedRoute,
    private router: Router
  ) {
    setTimeout(() => {
      this.rdt = connectionService.getRdt();
    }, 500);
  }
  async ngOnInit() {
    this.accountAddress = await this.connectionService.getAccount();
    this.checkAdmin();
  }
  // ngAfterViewInit() {
  //   console.log(this.dialog.nativeElement.value);
  // }
  public checkAdmin() {
    this.accountService
      .getFungiblesInfo(this.accountAddress[0].address)
      .then(async (resp: AccountFungibleInfo[]) => {
        const adminAddress =
          await this.radinsuranceInfoService.getAdminBadgeResourceAddress();
        resp.forEach((elm: AccountFungibleInfo) => {
          if (elm.fungibleAdress === adminAddress) {
            this.isAdmin = true;
          }
        });
      });
  }
  public toggleModal() {
    this.showDialog = !this.showDialog;
  }
  public generateComponentAdress() {
    this.connectionService.getAccount().then(async (resp) => {
      const address = resp[0].address;
      this.instanciateRadInsurance(address);
    });
  }
  public async instanciateRadInsurance(accountAdress: string) {
    let manifest =
      this.radInsuranceComponentManifestService.getRadInsuranceComponentManifest(
        accountAdress
      );
    let hash = await this.invokeManifest(manifest);
    let radInsuranceComponentAddress =
      await this.radinsuranceInfoService.getComponentAddressFromHash(hash);
    localStorage.setItem(
      RADINFO.RAD_INSURANCE_COMPONENT_LOCAL_STORAGE_NAME,
      radInsuranceComponentAddress
    );
    window.location.reload();
  }
  private async invokeManifest(manifest: string): Promise<string> {
    console.log('init component manifest : ', manifest);

    const result = await this.rdt.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });

    if (result.isErr()) {
      console.log('transaction id : ', result.error);
      throw result.error;
    }

    console.log('Success : ', result);
    return result.value.transactionIntentHash;
  }
  public createPolicy() {
    this.router.navigate(['create-policy']);
  }
}
