import { Injectable } from '@angular/core';
import { Account, RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit';
import { Observable } from 'rxjs';
import { Rdt } from 'src/app/types';

@Injectable({
  providedIn: 'root',
})
export class ConnectionService {
  private rdt!: Rdt;
  private accounts!: Account[];
  private ObservAccounts = new Observable<Account[]>((observer) => {
    this.rdt = RadixDappToolkit(
      {
        dAppDefinitionAddress:
          'account_tdx_b_1pp2p0sgkcg3zlnxtxemfeakx9985wjz6fyj5479hzj2sf4vg2l',
        dAppName: 'RadInsurance',
      },
      (requestData) => {
        requestData({
          accounts: { quantifier: 'atLeast', quantity: 1 },
        }).map((response) => {
          // set your application state
          this.accounts = response.data.accounts;
          observer.next(this.accounts);
          //localStorage.setItem("rdtAccountsData",this.accounts[0].address );
        });
      },
      {
        networkId: 11,
        onDisconnect: () => {
          // clear your application state
          this.accounts = [];
          observer.next(this.accounts);
        },
        onInit: ({ accounts }) => {
          // set your initial application state
          this.accounts = accounts ?? [];
          observer.next(this.accounts);
        },
      }
    );
  });

  constructor() {}

  public getRdt(): Rdt {
    return this.rdt;
  }
  public getAccount(): Promise<Account[]> {
    return new Promise<Account[]>((resolve, reject) => {
      this.ObservAccounts.subscribe((resp) => {
        resolve(resp);
      });
    });
  }
}
