import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ApiPolicies } from '../api/policies';
import { Policies, Policy } from './components/types';
import { map } from 'rxjs';
import { PolicyService } from '../services/policy.service';
import { RadinsuranceInfoService } from '../services/radinsurance-info.service';
import { RadInsurancePolicy } from '../types';
import { RADINFO } from '../consts';

@Injectable({
  providedIn: 'root',
})
export class HomeService {
  private Home_Mock_URL: string = '/assets/homeMock.json';

  constructor(
    private policyService: PolicyService,
    private radinsuranceInfoService: RadinsuranceInfoService
  ) {}

  public async getPolicies(): Promise<Policies> {
    let radInsuranceComponentInfo =  await this.radinsuranceInfoService.getRadInsuranceComponentInfo(); 
    const result: Policies = [];
    const radInsuranceInfoService =
    radInsuranceComponentInfo.policies;
    const allPolicy: RadInsurancePolicy[] = [];
    radInsuranceInfoService.forEach((element: RadInsurancePolicy) => {
      allPolicy.push(element);
    });
    allPolicy.forEach(async (element: RadInsurancePolicy) => {
      const policy = await this.policyService.getPolicyInfo(element);
      result.push({
        name: policy.name,
        description: policy.description,
        tokenCoveredCount: policy.totalInsuredsCoverAmount,
        id: policy.id,
      });
    });
    return new Promise((resolve, reject) => {
      resolve(result);
    });
  }
}
