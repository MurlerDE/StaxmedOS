import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Tenant } from '../models/Tenant';

@Injectable({ providedIn: 'root' })
export class TenantService {
  constructor(private http: HttpClient) {}

  getTenantInfo() {
    return this.http.get<Tenant>('/api/v1/tenant/me');
  }
}
