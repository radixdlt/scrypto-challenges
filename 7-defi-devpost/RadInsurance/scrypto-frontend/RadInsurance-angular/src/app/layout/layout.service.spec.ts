import { TestBed } from '@angular/core/testing';

import { AppLayoutService } from './layout.service';

describe('AppLayoutService', () => {
  let service: AppLayoutService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AppLayoutService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
