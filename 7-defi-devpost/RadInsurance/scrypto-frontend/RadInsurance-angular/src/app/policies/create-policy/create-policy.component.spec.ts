import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CreatePolicyComponent } from './create-policy.component';

describe('CreatePolicyComponent', () => {
  let component: CreatePolicyComponent;
  let fixture: ComponentFixture<CreatePolicyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ CreatePolicyComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CreatePolicyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
