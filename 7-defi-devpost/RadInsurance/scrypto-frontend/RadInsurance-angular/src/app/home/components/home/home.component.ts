import { Component, OnInit } from '@angular/core';
import { ApiPolicies, ApiPolicy } from 'src/app/api/policies';
import { HomeService } from '../../home.service';
import { Policies, Policy } from '../types';
import { Router, ActivatedRoute, ParamMap } from '@angular/router';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.css'],
})
export class HomeComponent implements OnInit {
  policies: Policies = [];
  constructor(
    private readonly homeService: HomeService,
    private router: Router
  ) {}

  ngOnInit(): void {
    this.init();
  }

  init() {
    this.homeService.getPolicies().then((pol: Policies) => {
      this.policies = pol;
    });
  }

  onInvestAsInsurer(id: number) {
    this.router.navigate(['invest-as-insurer', { id: id }]);
  }
}
