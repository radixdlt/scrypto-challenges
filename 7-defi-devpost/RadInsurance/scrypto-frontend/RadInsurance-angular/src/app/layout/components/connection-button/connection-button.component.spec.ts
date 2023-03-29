import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConnectionButtonComponent } from './connection-button.component';

describe('ConnectionButtonComponent', () => {
  let component: ConnectionButtonComponent;
  let fixture: ComponentFixture<ConnectionButtonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ConnectionButtonComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConnectionButtonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
