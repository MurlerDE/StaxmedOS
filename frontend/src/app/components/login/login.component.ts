import { Component, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AuthService } from '../../services/auth.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [FormsModule],
  template: `
    <form (ngSubmit)="onLogin()" class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700">Email / Username</label>
        <input
          [(ngModel)]="username"
          name="username"
          type="text"
          class="w-full p-2 border rounded-lg focus:ring-2 focus:ring-amber-500 outline-none"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700">Passwort</label>
        <input
          [(ngModel)]="password"
          name="password"
          type="password"
          class="w-full p-2 border rounded-lg focus:ring-2 focus:ring-amber-500 outline-none"
        />
      </div>
      <button
        type="submit"
        [disabled]="loading()"
        class="w-full bg-amber-500 text-white py-2 rounded-lg font-bold hover:bg-amber-600 disabled:bg-gray-400"
      >
        {{ loading() ? 'Prüfe...' : 'Anmelden' }}
      </button>
      @if (error()) {
        <p class="text-red-500 text-sm text-center">{{ error() }}</p>
      }
    </form>
  `,
})
export class LoginComponent {
  private auth = inject(AuthService);
  private router = inject(Router);

  username = '';
  password = '';
  loading = signal(false);
  error = signal('');

  onLogin() {
    this.loading.set(true);
    this.error.set('');

    this.auth.login({ username: this.username, password: this.password }).subscribe({
      next: () => {
        this.auth.showLoginModal.set(false); // Modal schließen!
        this.router.navigate(['/']); // Nach Login zum Dashboard
      },
      error: () => {
        this.error.set('Login fehlgeschlagen. Bitte Daten prüfen.');
        this.loading.set(false);
      },
    });
  }
}
