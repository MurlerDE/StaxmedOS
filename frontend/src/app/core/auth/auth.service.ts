import { Injectable, signal, inject, computed } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { tap } from 'rxjs';
import { UserInfo } from '../models/UserInfo';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private http = inject(HttpClient);
  private router = inject(Router);
  private readonly API_URL = '/api/v1/auth';

  // Signals für den Auth-Status
  currentUser = signal<{ username: string; role: string } | null>(null);
  token = signal<string | null>(null);

  isLoggedIn = computed(() => this.currentUser() !== null);
  isAdmin = computed(() => this.currentUser()?.role === 'admin');
  showLoginModal = signal(false);

  constructor() {
    // Beim Start: Wenn Token vorhanden, User-Daten wiederherstellen
    this.initializeAuth();
  }

  private initializeAuth() {
    const token = localStorage.getItem('stax_token');
    if (token) {
      // Token Signal setzen
      this.token.set(token);
      // Erst User-Daten aus localStorage laden (sofort verfügbar)
      const userData = localStorage.getItem('stax_user');
      if (userData) {
        try {
          this.currentUser.set(JSON.parse(userData));
        } catch (e) {
          console.error('Failed to parse user data from localStorage', e);
        }
      }

      // Dann im Hintergrund validieren (ohne zu blockieren)
      this.http.get<UserInfo>(`${this.API_URL}/me`).subscribe({
        next: (user) => {
          // Token ist gültig, User-Daten aktualisieren
          this.currentUser.set({ username: user.username, role: user.role });
          localStorage.setItem(
            'stax_user',
            JSON.stringify({ username: user.username, role: user.role }),
          );
        },
        error: (err) => {
          // Nur bei 401 (Unauthorized) ausloggen, andere Fehler ignorieren
          if (err.status === 401) {
            console.warn('Token expired or invalid, logging out');
            this.logout();
          } else {
            // Netzwerkfehler etc. - User bleibt eingeloggt
            console.warn('Failed to validate token, but keeping user logged in', err);
          }
        },
      });
    }
  }

  login(credentials: { username: string; password: string }) {
    return this.http.post<any>(`${this.API_URL}/login`, credentials).pipe(
      tap((res) => {
        this.token.set(res.token);
        const userData = { username: res.username, role: res.role };
        this.currentUser.set(userData);
        localStorage.setItem('stax_token', res.token);
        localStorage.setItem('stax_user', JSON.stringify(userData));
      }),
    );
  }

  activeModules = signal(['inventory', 'compliance']);

  logout() {
    this.token.set(null);
    this.currentUser.set(null);
    localStorage.removeItem('stax_token');
    localStorage.removeItem('stax_user');
    this.router.navigate(['/']);
  }
}
