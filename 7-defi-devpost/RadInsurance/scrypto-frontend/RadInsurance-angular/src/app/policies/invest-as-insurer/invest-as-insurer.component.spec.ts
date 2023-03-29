import { ComponentFixture, TestBed } from '@angular/core/testing';

import { InvestAsInsurerComponent } from './invest-as-insurer.component';

describe('InvestAsInsurerComponent', () => {
  let component: InvestAsInsurerComponent;
  let fixture: ComponentFixture<InvestAsInsurerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ InvestAsInsurerComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(InvestAsInsurerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
