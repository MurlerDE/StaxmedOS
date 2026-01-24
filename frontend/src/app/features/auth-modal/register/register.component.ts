import { Component, signal, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AlertComponent } from '../../../shared/components/alert/alert.component';
import { AuthService } from '../../../core/services/auth.service';
import { ModalService } from '../../../core/services/modal.service';

@Component({
  selector: 'app-register',
  imports: [FormsModule, AlertComponent],
  templateUrl: './register.component.html',
  styleUrl: './register.component.css',
})
export class RegisterComponent {
  auth = inject(AuthService);
  modalService = inject(ModalService);
  router = inject(Router);

  credentials = {
    username: '',
    password: '',
    confirmPassword: '',
    firstName: '',
    lastName: '',
  };

  inSubmission = signal(false);
  showAlert = signal(false);
  alertMsg = signal('Bitte warten! Wir erstellen Ihr Konto.');
  alertColor = signal('blue');

  register() {
    // Passwort-Übereinstimmung prüfen
    if (this.credentials.password !== this.credentials.confirmPassword) {
      this.showAlert.set(true);
      this.alertMsg.set('Die Passwörter stimmen nicht überein.');
      this.alertColor.set('red');
      return;
    }

    this.inSubmission.set(true);
    this.showAlert.set(true);
    this.alertMsg.set('Bitte warten! Wir erstellen Ihr Konto.');
    this.alertColor.set('blue');

    const registerData = {
      username: this.credentials.username,
      password: this.credentials.password,
      first_name: this.credentials.firstName || undefined,
      last_name: this.credentials.lastName || undefined,
    };

    this.auth.register(registerData).subscribe({
      next: (res) => {
        this.alertMsg.set('Ihr Konto wurde erfolgreich erstellt! Sie werden jetzt angemeldet...');
        this.alertColor.set('green');

        // Nach erfolgreicher Registrierung automatisch einloggen
        setTimeout(() => {
          this.auth.login(registerData).subscribe({
            next: () => {
              this.modalService.toggle('auth');
              this.router.navigate(['/dashboard']);
            },
            error: () => {
              this.alertMsg.set('Konto erstellt. Bitte melden Sie sich an.');
              this.alertColor.set('green');
              setTimeout(() => {
                this.modalService.toggle('auth');
              }, 2000);
            },
          });
        }, 1000);
      },
      error: (err) => {
        this.inSubmission.set(false);
        const errorMsg =
          err.error?.message || 'Benutzername bereits vergeben oder ungültige Eingabe.';
        this.alertMsg.set(errorMsg);
        this.alertColor.set('red');
      },
    });
  }
}
