import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { LayoutModule } from './layout/layout.module';
import { HomeComponent } from './home/components/home/home.component';
import { HomeService } from './home/home.service';
import { HttpClientModule } from '@angular/common/http';
import { RadinsuranceInfoService } from './services/radinsurance-info.service';
import { InvestAsInsurerComponent } from './policies/invest-as-insurer/invest-as-insurer.component';
import { CreatePolicyComponent } from './policies/create-policy/create-policy.component';
import { FormsModule } from '@angular/forms';
import { ConnectionService } from 'src/app/layout/components/connection-button/connection.service';
import { AccountService } from 'src/app/services/account.service';

@NgModule({
  declarations: [
    AppComponent,
    HomeComponent,
    InvestAsInsurerComponent,
    CreatePolicyComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    LayoutModule,
    HttpClientModule,
    FormsModule,
  ],
  providers: [HomeService, RadinsuranceInfoService, ConnectionService, AccountService],
  bootstrap: [AppComponent],
})
export class AppModule {}
