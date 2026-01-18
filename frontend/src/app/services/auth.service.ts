import { Injectable, signal, inject, computed } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { tap } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private http = inject(HttpClient);
  private readonly API_URL = 'http://127.0.0.1:3000/api/auth';

  // Signals für den Auth-Status
  currentUser = signal<{ username: string; role: string } | null>(null);
  token = signal<string | null>(localStorage.getItem('stax_token'));

  isLoggedIn = computed(() => this.currentUser() !== null);
  isAdmin = computed(() => this.currentUser()?.role === 'admin');
  showLoginModal = signal(false);

  login(credentials: { username: string; password: string }) {
    return this.http.post<any>(`${this.API_URL}/login`, credentials).pipe(
      tap((res) => {
        this.token.set(res.token);
        this.currentUser.set({ username: res.username, role: res.role });
        localStorage.setItem('stax_token', res.token);
      }),
    );
  }

  activeModules = signal(['inventory', 'compliance']);

  logout() {
    this.token.set(null);
    this.currentUser.set(null);
    localStorage.removeItem('stax_token');
  }
}
