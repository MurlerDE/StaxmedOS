import { Injectable, signal, inject, computed } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { BehaviorSubject, Observable, tap, catchError, of } from 'rxjs';
import { UserInfo } from '../models/UserInfo';

/**
 * Vollständige User-Daten mit allen Feldern
 */
export interface User {
  username: string;
  role: string;
  user_id: string;
  tenant_id: string | null;
  first_name: string | null;
  last_name: string | null;
}

/**
 * Login-Response vom Backend
 */
interface LoginResponse {
  token: string;
  username: string;
  role: string;
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private http = inject(HttpClient);
  private router = inject(Router);
  private readonly API_URL = '/api/v1/auth';
  private readonly TOKEN_KEY = 'stax_token';
  private readonly USER_KEY = 'stax_user';

  // BehaviorSubject für reaktive User-Daten (kompatibel mit Observables)
  private userSubject = new BehaviorSubject<User | null>(this.getStoredUser());

  // Observable für Template-Subscriptions
  public user$ = this.userSubject.asObservable();

  // Signals für moderne Angular-Templates
  currentUser = signal<User | null>(this.userSubject.value);
  token = signal<string | null>(this.getStoredToken());

  // Computed Signals für Berechtigungen
  isLoggedIn = computed(() => this.currentUser() !== null);
  isAdmin = computed(() => {
    const role = this.currentUser()?.role;
    return role === 'admin' || role === 'staxadmin';
  });
  isStaxAdmin = computed(() => this.currentUser()?.role === 'staxadmin');
  isTenantAdmin = computed(() => this.currentUser()?.role === 'admin');
  isLogisticManager = computed(() => this.currentUser()?.role === 'logistic_manager');
  isParamedic = computed(() => this.currentUser()?.role === 'paramedic');

  // Berechnete Display-Namen
  displayName = computed(() => {
    const user = this.currentUser();
    if (!user) return '';

    const firstName = user.first_name?.trim();
    const lastName = user.last_name?.trim();
    if (firstName && lastName) {
      return `${firstName} ${lastName}`;
    }
    if (firstName) return firstName;
    if (lastName) return lastName;
    return user.username;
  });

  initials = computed(() => {
    const user = this.currentUser();
    if (!user) return '';

    const firstName = user.first_name?.trim();
    const lastName = user.last_name?.trim();

    if (firstName && lastName) {
      return `${firstName[0]}${lastName[0]}`.toUpperCase();
    }
    if (firstName) return firstName[0].toUpperCase();
    return user.username[0].toUpperCase();
  });

  // Aktive Module (später aus Backend laden)
  activeModules = signal(['inventory', 'compliance']);

  constructor() {
    // Synchronisiere Signal mit BehaviorSubject
    this.userSubject.subscribe((user) => {
      this.currentUser.set(user);
    });

    // Token-Validierung beim Start
    this.initializeAuth();
  }

  /**
   * Lädt gespeicherten Token aus localStorage
   */
  private getStoredToken(): string | null {
    return localStorage.getItem(this.TOKEN_KEY);
  }

  /**
   * Lädt gespeicherte User-Daten aus localStorage
   */
  private getStoredUser(): User | null {
    const data = localStorage.getItem(this.USER_KEY);
    if (!data) return null;

    try {
      return JSON.parse(data);
    } catch (e) {
      console.error('Failed to parse stored user data:', e);
      return null;
    }
  }

  /**
   * Initialisiert Auth-Status beim App-Start
   * Validiert Token im Hintergrund ohne UI zu blockieren
   */
  private initializeAuth(): void {
    const token = this.getStoredToken();
    if (!token) return;

    // User-Daten sind bereits aus localStorage geladen
    // Jetzt im Hintergrund validieren
    this.http.get<UserInfo>(`${this.API_URL}/me`).subscribe({
      next: (userInfo) => {
        // Token ist gültig, User-Daten aktualisieren
        const user: User = {
          username: userInfo.username,
          role: userInfo.role,
          user_id: userInfo.user_id,
          tenant_id: userInfo.tenant_id,
          first_name: userInfo.first_name || null,
          last_name: userInfo.last_name || null,
        };

        this.updateUser(user);
        localStorage.setItem(this.USER_KEY, JSON.stringify(user));
      },
      error: (err) => {
        // Nur bei 401 (Unauthorized) ausloggen
        if (err.status === 401) {
          console.warn('Token expired or invalid, logging out');
          this.logout();
        } else {
          // Netzwerkfehler etc. - User bleibt eingeloggt mit gecachten Daten
          console.warn('Failed to validate token, keeping cached user data', err);
        }
      },
    });
  }

  /**
   * Aktualisiert User-Daten in allen Systemen
   */
  private updateUser(user: User | null): void {
    this.userSubject.next(user);
    this.currentUser.set(user);
  }

  /**
   * Login mit Credentials
   */
  login(credentials: { username: string; password: string }): Observable<LoginResponse> {
    return this.http.post<LoginResponse>(`${this.API_URL}/login`, credentials).pipe(
      tap((res) => {
        // Token speichern
        this.token.set(res.token);
        localStorage.setItem(this.TOKEN_KEY, res.token);

        // Basis User-Daten aus Login-Response
        const basicUser: User = {
          username: res.username,
          role: res.role,
          user_id: '', // Wird von /me endpoint gefüllt
          tenant_id: null,
          first_name: null,
          last_name: null,
        };

        this.updateUser(basicUser);
        localStorage.setItem(this.USER_KEY, JSON.stringify(basicUser));

        // Vollständige Daten im Hintergrund laden
        this.loadFullUserData();
      }),
    );
  }

  /**
   * Registrierung eines neuen Users
   */
  register(credentials: { username: string; password: string }): Observable<void> {
    return this.http.post<void>(`${this.API_URL}/register`, credentials);
  }

  /**
   * Lädt vollständige User-Daten vom /me Endpoint
   */
  private loadFullUserData(): void {
    this.http.get<UserInfo>(`${this.API_URL}/me`).subscribe({
      next: (userInfo) => {
        const user: User = {
          username: userInfo.username,
          role: userInfo.role,
          user_id: userInfo.user_id,
          tenant_id: userInfo.tenant_id,
          first_name: userInfo.first_name || null,
          last_name: userInfo.last_name || null,
        };

        this.updateUser(user);
        localStorage.setItem(this.USER_KEY, JSON.stringify(user));
      },
      error: (err) => {
        console.error('Failed to load full user data:', err);
      },
    });
  }

  /**
   * Logout und Cleanup
   */
  logout(): void {
    this.token.set(null);
    this.updateUser(null);
    localStorage.removeItem(this.TOKEN_KEY);
    localStorage.removeItem(this.USER_KEY);
    this.router.navigate(['/']);
  }

  /**
   * Aktueller User (synchron)
   * Für Verwendung außerhalb von Templates
   */
  get currentUserValue(): User | null {
    return this.userSubject.value;
  }

  /**
   * Prüft ob User eine bestimmte Rolle hat
   */
  hasRole(role: string): boolean {
    return this.currentUserValue?.role === role;
  }

  /**
   * Prüft ob User eine der angegebenen Rollen hat
   */
  hasAnyRole(roles: string[]): boolean {
    const userRole = this.currentUserValue?.role;
    return userRole ? roles.includes(userRole) : false;
  }

  /**
   * Prüft ob User eine bestimmte Permission hat
   * TODO: Später vom Backend laden, aktuell Placeholder basierend auf Rolle
   */
  hasPermission(permission: string): boolean {
    const user = this.currentUserValue;
    if (!user) return false;

    // StaxAdmin hat alle Permissions
    if (user.role === 'staxadmin') return true;

    // Admin hat die meisten Permissions (außer system.*)
    if (user.role === 'admin') {
      if (permission.startsWith('system.')) return false;
      return true;
    }

    // Logistic Manager
    if (user.role === 'logistic_manager') {
      const allowedPermissions = [
        'inventory.view.all',
        'inventory.create',
        'inventory.update',
        'shipments.view',
        'shipments.create',
        'shipments.receive',
        'shipments.manage',
        'locations.view',
        'missions.view',
      ];
      return allowedPermissions.includes(permission);
    }

    // Paramedic
    if (user.role === 'paramedic') {
      const allowedPermissions = ['inventory.view.own', 'missions.report', 'locations.view'];
      return allowedPermissions.includes(permission);
    }

    return false;
  }

  /**
   * Prüft ob User eine der angegebenen Permissions hat
   */
  hasAnyPermission(permissions: string[]): boolean {
    return permissions.some((p) => this.hasPermission(p));
  }

  /**
   * Aktualisiert User-Profil (z.B. nach Settings-Änderung)
   */
  refreshUserData(): Observable<User> {
    return this.http.get<UserInfo>(`${this.API_URL}/me`).pipe(
      tap((userInfo) => {
        const user: User = {
          username: userInfo.username,
          role: userInfo.role,
          user_id: userInfo.user_id,
          tenant_id: userInfo.tenant_id,
          first_name: userInfo.first_name || null,
          last_name: userInfo.last_name || null,
        };

        this.updateUser(user);
        localStorage.setItem(this.USER_KEY, JSON.stringify(user));
      }),
      catchError((err) => {
        console.error('Failed to refresh user data:', err);
        throw err;
      }),
    );
  }
}

/**
 * Aktualisiert User-Profil (z.B. nach Settings-Änderung)
 * <!-- Display Name -->
 * <p>Willkommen, {{ auth.displayName() }}!</p>
 *
 * <!-- Avatar mit Initialen -->
 * <div class="avatar">{{ auth.initials() }}</div>
 *
 * <!-- Rollen-Check -->
 * @if (auth.isStaxAdmin()) {
 *   <button>System-Verwaltung</button>
 * }
 *
 * <!-- Observable -->
 * @if (auth.user$ | async; as user) {
 *   <p>{{ user.first_name }} {{ user.last_name }}</p>
 * }
 *
 */
