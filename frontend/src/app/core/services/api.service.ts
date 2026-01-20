// frontend/src/app/core/services/api.service.ts
import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class ApiService {
  // Zentraler Prefix für alle Anfragen
  private readonly baseUrl = '/api/v1';

  // Beispiel Methode
  // getInventory() {
  //  return this.http.get(`${this.baseUrl}/inventory`);
  // }
}
