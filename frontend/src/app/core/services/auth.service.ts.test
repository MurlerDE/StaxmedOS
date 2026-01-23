import { Injectable, inject } from '@angular/common/http';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject, Observable, tap } from 'rxjs';
import { Router } from '@angular/router';

export interface User {
  token: string;
  username: string;
  role: string;
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private http = inject(HttpClient);
  private router = inject(Router);
  private readonly AUTH_KEY = 'app_auth_token';

  // Der "Firebase-Style" State
  private userSubject = new BehaviorSubject<User | null>(this.getStoredUser());
  public user$ = this.userSubject.asObservable();

  private getStoredUser(): User | null {
    const data = localStorage.getItem(this.AUTH_KEY);
    return data ? JSON.parse(data) : null;
  }

  register(credentials: any): Observable<any> {
    return this.http.post('/api/register', credentials);
  }

  login(credentials: any): Observable<User> {
    return this.http.post<User>('/api/login', credentials).pipe(
      tap(user => {
        localStorage.setItem(this.AUTH_KEY, JSON.stringify(user));
        this.userSubject.next(user);
      })
    );
  }

  logout() {
    localStorage.removeItem(this.AUTH_KEY);
    this.userSubject.next(null);
    this.router.navigate(['/login']);
  }

  get currentUserValue(): User | null {
    return this.userSubject.value;
  }

  get isAuthenticated(): boolean {
    return !!this.userSubject.value;
  }
}
