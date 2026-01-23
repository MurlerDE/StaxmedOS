import { Component } from '@angular/core';
import { ModalComponent } from '../../shared/components/modal/modal.component';
import { TabComponent } from '../../shared/components/tab/tab.component';
import { TabsContainerComponent } from '../../shared/components/tabs-container/tabs-container.component';
import { LoginComponent } from './login/login.component';
import { RegisterComponent } from './register/register.component';

@Component({
  selector: 'app-auth-modal',
  imports: [
    ModalComponent,
    TabsContainerComponent,
    TabComponent,
    LoginComponent,
    RegisterComponent,
  ],
  templateUrl: './auth-modal.component.html',
  styleUrl: './auth-modal.component.css',
})
export class AuthModalComponent {}
