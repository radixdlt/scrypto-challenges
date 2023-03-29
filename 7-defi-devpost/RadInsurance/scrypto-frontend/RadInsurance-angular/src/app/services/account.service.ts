import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { RADINFO } from '../consts';
import { AccountFungibleInfo } from '../types';

@Injectable({
  providedIn: 'root',
})
export class AccountService {
  private radInfo = RADINFO;
  private entityResourceUrl = `${this.radInfo.BASE_BETANET_URL}/entity/resources`;

  constructor(private http: HttpClient) {}

  public async getEntityRessource(address: string): Promise<any> {
    return new Promise((resolve, reject) => {
      this.http
        .post(this.entityResourceUrl, {
          address: address,
        })
        .subscribe((resp) => resolve(resp));
    });
  }

  public async getFungiblesInfo(
    AccountAddress: string
  ): Promise<AccountFungibleInfo[]> {
    const entityResource = await this.getEntityRessource(AccountAddress);
    const result: AccountFungibleInfo[] = [];
    entityResource.fungible_resources.items.forEach((item: any) => {
      const isXRD: boolean = item.address === this.radInfo.XRD_RESOURCE_ADDRESS;
      result.push(
        new AccountFungibleInfo(item.address, item.amount.value, isXRD)
      );
    });
    return new Promise((resolve, reject) => {
      resolve(result);
    });
  }
}
