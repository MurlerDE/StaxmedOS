import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import type {
  TenantListItem,
  TenantDetail,
  CreateTenantRequest,
  UpdateTenantRequest,
  TenantQueryParams,
  TenantAddress,
  CreateAddressRequest,
  UpdateAddressRequest,
  TenantContact,
  CreateContactRequest,
  UpdateContactRequest,
  TenantPayment,
  CreatePaymentRequest,
  UpdatePaymentRequest,
  Plan,
  SwitchPlanRequest,
  TenantStatistics,
} from '../models';

@Injectable({
  providedIn: 'root',
})
export class AdminService {
  private http = inject(HttpClient);
  private apiUrl = '/api/v1/admin';

  // ============================================================================
  // TENANT MANAGEMENT
  // ============================================================================

  /**
   * Liste aller Tenants mit optionalen Filtern und Paginierung
   */
  listTenants(params?: {
    status?: string;
    plan_id?: string;
    search?: string;
    page?: number;
    per_page?: number;
  }): Observable<TenantListItem[]> {
    let httpParams = new HttpParams();
    if (params?.status) httpParams = httpParams.set('status', params.status);
    if (params?.plan_id) httpParams = httpParams.set('plan_id', params.plan_id);
    if (params?.search) httpParams = httpParams.set('search', params.search);
    if (params?.page) httpParams = httpParams.set('page', params.page.toString());
    if (params?.per_page) httpParams = httpParams.set('per_page', params.per_page.toString());

    return this.http.get<TenantListItem[]>(`${this.apiUrl}/tenants`, { params: httpParams });
  }

  /**
   * Einzelnen Tenant mit allen Details abrufen
   */
  getTenant(id: string): Observable<TenantDetail> {
    return this.http.get<TenantDetail>(`${this.apiUrl}/tenants/${id}`);
  }

  /**
   * Neuen Tenant anlegen
   */
  createTenant(data: CreateTenantRequest): Observable<TenantDetail> {
    return this.http.post<TenantDetail>(`${this.apiUrl}/tenants`, data);
  }

  /**
   * Tenant aktualisieren
   */
  updateTenant(id: string, data: UpdateTenantRequest): Observable<TenantDetail> {
    return this.http.put<TenantDetail>(`${this.apiUrl}/tenants/${id}`, data);
  }

  /**
   * Tenant löschen
   */
  deleteTenant(id: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/tenants/${id}`);
  }

  /**
   * Tenant-Status ändern
   */
  updateTenantStatus(id: string, status: string): Observable<TenantDetail> {
    return this.http.patch<TenantDetail>(`${this.apiUrl}/tenants/${id}/status`, { status });
  }

  // ============================================================================
  // PLAN MANAGEMENT
  // ============================================================================

  /**
   * Alle verfügbaren Pläne abrufen
   */
  listPlans(): Observable<Plan[]> {
    return this.http.get<Plan[]>(`${this.apiUrl}/plans`);
  }

  /**
   * Plan eines Tenants wechseln
   */
  switchTenantPlan(tenantId: string, planId: string): Observable<TenantDetail> {
    return this.http.put<TenantDetail>(`${this.apiUrl}/tenants/${tenantId}/plan`, {
      plan_id: planId,
    });
  }

  // ============================================================================
  // ADDRESS MANAGEMENT
  // ============================================================================

  /**
   * Alle Adressen eines Tenants abrufen
   */
  listTenantAddresses(tenantId: string): Observable<TenantAddress[]> {
    return this.http.get<TenantAddress[]>(`${this.apiUrl}/tenants/${tenantId}/addresses`);
  }

  /**
   * Adresse zu Tenant hinzufügen
   */
  createTenantAddress(tenantId: string, data: CreateAddressRequest): Observable<TenantAddress> {
    return this.http.post<TenantAddress>(`${this.apiUrl}/tenants/${tenantId}/addresses`, data);
  }

  /**
   * Adresse aktualisieren
   */
  updateTenantAddress(
    tenantId: string,
    addressId: string,
    data: UpdateAddressRequest,
  ): Observable<TenantAddress> {
    return this.http.put<TenantAddress>(
      `${this.apiUrl}/tenants/${tenantId}/addresses/${addressId}`,
      data,
    );
  }

  /**
   * Adresse löschen
   */
  deleteTenantAddress(tenantId: string, addressId: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/tenants/${tenantId}/addresses/${addressId}`);
  }

  // ============================================================================
  // CONTACT MANAGEMENT
  // ============================================================================

  /**
   * Alle Kontakte eines Tenants abrufen
   */
  listTenantContacts(tenantId: string): Observable<TenantContact[]> {
    return this.http.get<TenantContact[]>(`${this.apiUrl}/tenants/${tenantId}/contacts`);
  }

  /**
   * Kontakt zu Tenant hinzufügen
   */
  createTenantContact(tenantId: string, data: CreateContactRequest): Observable<TenantContact> {
    return this.http.post<TenantContact>(`${this.apiUrl}/tenants/${tenantId}/contacts`, data);
  }

  /**
   * Kontakt aktualisieren
   */
  updateTenantContact(
    tenantId: string,
    contactId: string,
    data: UpdateContactRequest,
  ): Observable<TenantContact> {
    return this.http.put<TenantContact>(
      `${this.apiUrl}/tenants/${tenantId}/contacts/${contactId}`,
      data,
    );
  }

  /**
   * Kontakt löschen
   */
  deleteTenantContact(tenantId: string, contactId: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/tenants/${tenantId}/contacts/${contactId}`);
  }

  // ============================================================================
  // PAYMENT MANAGEMENT
  // ============================================================================

  /**
   * Alle Zahlungen eines Tenants abrufen
   */
  listTenantPayments(tenantId: string): Observable<TenantPayment[]> {
    return this.http.get<TenantPayment[]>(`${this.apiUrl}/tenants/${tenantId}/payments`);
  }

  /**
   * Zahlung erfassen
   */
  createTenantPayment(tenantId: string, data: CreatePaymentRequest): Observable<TenantPayment> {
    return this.http.post<TenantPayment>(`${this.apiUrl}/tenants/${tenantId}/payments`, data);
  }

  /**
   * Zahlung aktualisieren (Status, paid_at, notes)
   */
  updateTenantPayment(
    tenantId: string,
    paymentId: string,
    data: UpdatePaymentRequest,
  ): Observable<TenantPayment> {
    return this.http.put<TenantPayment>(
      `${this.apiUrl}/tenants/${tenantId}/payments/${paymentId}`,
      data,
    );
  }

  /**
   * Zahlung löschen
   */
  deleteTenantPayment(tenantId: string, paymentId: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/tenants/${tenantId}/payments/${paymentId}`);
  }

  /**
   * Zahlung als bezahlt markieren
   */
  markPaymentAsPaid(tenantId: string, paymentId: string): Observable<TenantPayment> {
    return this.updateTenantPayment(tenantId, paymentId, {
      payment_status: 'paid',
      paid_at: new Date().toISOString(),
      notes: null,
    });
  }

  // ============================================================================
  // STATISTICS
  // ============================================================================

  /**
   * Statistiken über alle Tenants abrufen
   */
  getStatistics(): Observable<TenantStatistics> {
    return this.http.get<TenantStatistics>(`${this.apiUrl}/statistics`);
  }

  // ============================================================================
  // HELPER METHODS
  // ============================================================================

  /**
   * Status-Badge-Farbe für Tenant-Status
   */
  getStatusBadgeColor(status: string): string {
    switch (status) {
      case 'active':
        return 'bg-green-100 text-green-800';
      case 'trial':
        return 'bg-blue-100 text-blue-800';
      case 'suspended':
        return 'bg-red-100 text-red-800';
      case 'canceled':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  }

  /**
   * Status-Badge-Text für Tenant-Status
   */
  getStatusText(status: string): string {
    switch (status) {
      case 'active':
        return 'Aktiv';
      case 'trial':
        return 'Testphase';
      case 'suspended':
        return 'Gesperrt';
      case 'canceled':
        return 'Gekündigt';
      default:
        return status;
    }
  }

  /**
   * Adresstyp-Text
   */
  getAddressTypeText(type: string): string {
    switch (type) {
      case 'billing':
        return 'Rechnungsadresse';
      case 'shipping':
        return 'Lieferadresse';
      case 'main':
        return 'Hauptadresse';
      case 'other':
        return 'Sonstige';
      default:
        return type;
    }
  }

  /**
   * Kontakttyp-Text
   */
  getContactTypeText(type: string): string {
    switch (type) {
      case 'primary':
        return 'Hauptansprechpartner';
      case 'billing':
        return 'Buchhaltung';
      case 'technical':
        return 'Technisch';
      case 'support':
        return 'Support';
      case 'other':
        return 'Sonstige';
      default:
        return type;
    }
  }

  /**
   * Zahlungsstatus-Badge-Farbe
   */
  getPaymentStatusColor(status: string): string {
    switch (status) {
      case 'paid':
        return 'bg-green-100 text-green-800';
      case 'pending':
        return 'bg-yellow-100 text-yellow-800';
      case 'overdue':
        return 'bg-red-100 text-red-800';
      case 'failed':
        return 'bg-red-100 text-red-800';
      case 'cancelled':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  }

  /**
   * Zahlungsstatus-Text
   */
  getPaymentStatusText(status: string): string {
    switch (status) {
      case 'paid':
        return 'Bezahlt';
      case 'pending':
        return 'Ausstehend';
      case 'overdue':
        return 'Überfällig';
      case 'failed':
        return 'Fehlgeschlagen';
      case 'cancelled':
        return 'Storniert';
      default:
        return status;
    }
  }

  /**
   * Datum formatieren
   */
  formatDate(dateString: string | null | undefined): string {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString('de-DE', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    });
  }

  /**
   * Datum und Zeit formatieren
   */
  formatDateTime(dateString: string | null | undefined): string {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString('de-DE', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  /**
   * Betrag formatieren
   */
  formatAmount(amount: string | number, currency: string = 'EUR'): string {
    const numAmount = typeof amount === 'string' ? parseFloat(amount) : amount;
    return new Intl.NumberFormat('de-DE', {
      style: 'currency',
      currency: currency,
    }).format(numAmount);
  }
}
