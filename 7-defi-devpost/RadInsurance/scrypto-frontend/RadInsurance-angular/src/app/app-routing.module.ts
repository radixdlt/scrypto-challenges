import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomeComponent } from './home/components/home/home.component';
import { InvestAsInsurerComponent } from './policies/invest-as-insurer/invest-as-insurer.component';
import { CreatePolicyComponent } from './policies/create-policy/create-policy.component';
const routes: Routes = [
  { path: 'home', component: HomeComponent },
  { path: 'invest-as-insurer', component: InvestAsInsurerComponent },
  { path: 'create-policy', component: CreatePolicyComponent },
  { path: '', redirectTo: 'home', pathMatch: 'full' },
  { path: '**', component: HomeComponent }, // Wildcard route for a 404 page
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
