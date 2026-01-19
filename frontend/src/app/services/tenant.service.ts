import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Tenant } from '../generated/Tenant'; // Automatisch generiert!

@Injectable({ providedIn: 'root' })
export class TenantService {
  constructor(private http: HttpClient) {}

  getTenantInfo() {
    return this.http.get<Tenant>('/api/tenant/me');
  }
}
