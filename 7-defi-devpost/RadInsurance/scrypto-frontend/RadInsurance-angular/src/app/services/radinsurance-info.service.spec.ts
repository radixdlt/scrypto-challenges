import { TestBed } from '@angular/core/testing';

import { RadinsuranceInfoService } from './radinsurance-info.service';

describe('RadinsuranceInfoService', () => {
  let service: RadinsuranceInfoService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(RadinsuranceInfoService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
