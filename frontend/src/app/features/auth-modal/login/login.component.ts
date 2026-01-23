import { Component, signal, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AlertComponent } from '../../../shared/components/alert/alert.component';
import { AuthService } from '../../../core/services/auth.service';
import { ModalService } from '../../../core/services/modal.service';

@Component({
  selector: 'app-login',
  imports: [FormsModule, AlertComponent],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
})
export class LoginComponent {
  auth = inject(AuthService);
  modalService = inject(ModalService);
  router = inject(Router);

  credentials = {
    username: '',
    password: '',
  };

  inSubmission = signal(false);
  showAlert = signal(false);
  alertMsg = signal('Bitte warten! Wir versuchen Sie anzumelden.');
  alertColor = signal('blue');

  login() {
    this.inSubmission.set(true);
    this.showAlert.set(true);
    this.alertMsg.set('Bitte warten! Wir versuchen Sie anzumelden.');
    this.alertColor.set('blue');

    this.auth.login(this.credentials).subscribe({
      next: (res) => {
        this.alertMsg.set('Sie wurden erfolgreich angemeldet!');
        this.alertColor.set('green');

        setTimeout(() => {
          this.modalService.toggle('auth');
          this.router.navigate(['/dashboard']);
        }, 1000);
      },
      error: (err) => {
        this.inSubmission.set(false);
        this.alertMsg.set('Etwas ist schief gelaufen. Bitte versuchen Sie es später noch einmal.');
        this.alertColor.set('red');
      },
    });
  }
}
