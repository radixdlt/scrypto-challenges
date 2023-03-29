import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FooterComponent } from './components/footer/footer.component';
import { MenuComponent } from './components/menu/menu.component';
import { TopbarComponent } from './components/topbar/topbar.component';
import { SidebarComponent } from './components/sidebar/sidebar.component';
import { ConnectionButtonComponent } from './components/connection-button/connection-button.component';
import { RouterModule } from '@angular/router';

@NgModule({
  declarations: [
    FooterComponent,
    MenuComponent,
    TopbarComponent,
    SidebarComponent,
    ConnectionButtonComponent,
  ],
  imports: [CommonModule, RouterModule],
  exports: [FooterComponent, MenuComponent, TopbarComponent, SidebarComponent],
})
export class LayoutModule {}
