import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class AppService {
  private readonly BETANET_URL = 'https://betanet.radixdlt.com';
  private readonly RAD_INSURANCE_COMPONENT_ADDRESS =
    'component_tdx_b_1qtskqpw8u9q0ugef3493glrmdh899cct8zkm6253u2nsc8gwht';
  private readonly RAD_INSURANCE_PACKAGE_ADDRESS =
    'package_tdx_b_1q8ap355se0ndadyxcck6c4ze3jkjp4czyyv78flu2x6sghcxkw';

  constructor(private http: HttpClient) {}

  public getRadixComponentInfo(): Observable<any> {
    const url = `${this.BETANET_URL}/entity/details`;
    return this.http.post<any>(url, {
      address:  this.RAD_INSURANCE_COMPONENT_ADDRESS,
    });

    //  result.details.state.data_json[0].forEach(policyIdAndAddressTab => {
    //   policies[policyIdAndAddressTab[0]] = policyIdAndAddressTab[1];
  }
}
