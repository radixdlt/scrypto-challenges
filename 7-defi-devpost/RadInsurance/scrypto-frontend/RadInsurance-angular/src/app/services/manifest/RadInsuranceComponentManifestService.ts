import { Injectable } from '@angular/core';
import { RADINFO } from 'src/app/consts';


@Injectable({
  providedIn: 'root',
})
export class RadInsuranceComponentManifestService {
  constructor() {}

  getRadInsuranceComponentManifest(ACOUNT_ADDRESS: string): string {
    let manifest = `
      CALL_FUNCTION PackageAddress("${RADINFO.RAD_INSURANCE_PACKAGE_ADDRESS}") "RadInsurance" "instanciate_rad_insurance" Decimal("${RADINFO.RAD_INSURANCE_FEE}") ResourceAddress("${RADINFO.XRD_RESOURCE_ADDRESS}");
      CALL_METHOD ComponentAddress("${ACOUNT_ADDRESS}") "deposit_batch" Expression("ENTIRE_WORKTOP");`;
    return manifest;
  }
}
