import { Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AdminService } from '../../../core/services/admin.service';
import type {
  TenantListItem,
  TenantDetail,
  CreateTenantRequest,
  UpdateTenantRequest,
  Plan,
  TenantStatistics,
} from '../../../core/models';

@Component({
  selector: 'app-tenants',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './tenants.component.html',
  styleUrls: ['./tenants.component.css'],
})
export class TenantsComponent implements OnInit {
  private adminService = inject(AdminService);
  private router = inject(Router);

  // State
  tenants = signal<TenantListItem[]>([]);
  filteredTenants = signal<TenantListItem[]>([]);
  selectedTenant = signal<TenantDetail | null>(null);
  plans = signal<Plan[]>([]);
  statistics = signal<TenantStatistics | null>(null);
  loading = signal(false);

  // Modals
  showCreateModal = signal(false);
  showEditModal = signal(false);
  showDeleteConfirm = signal(false);
  showPlanModal = signal(false);
  showDetailModal = signal(false);

  // Messages
  errorMessage = signal<string | null>(null);
  successMessage = signal<string | null>(null);

  // Filters
  filterStatus = '';
  filterPlan = '';
  searchQuery = '';
  currentPage = 1;
  perPage = 20;

  // Forms
  newTenant: CreateTenantRequest = {
    name: '',
    plan_id: '',
    status: 'trial',
    vat_number: '',
    tax_id: '',
    website: '',
    industry: '',
    company_size: '',
    preferred_language: 'de',
    notes: '',
  };

  editTenant: UpdateTenantRequest = {
    name: '',
    status: '',
    vat_number: '',
    tax_id: '',
    website: '',
    industry: '',
    company_size: '',
    preferred_language: '',
    notes: '',
  };

  selectedPlanId = '';
  tenantToDelete: TenantListItem | null = null;

  ngOnInit() {
    this.loadTenants();
    this.loadPlans();
    this.loadStatistics();
  }

  // ============================================================================
  // DATA LOADING
  // ============================================================================

  loadTenants() {
    this.loading.set(true);
    const params: any = {
      page: this.currentPage,
      per_page: this.perPage,
    };

    if (this.filterStatus) params.status = this.filterStatus;
    if (this.filterPlan) params.plan_id = this.filterPlan;
    if (this.searchQuery) params.search = this.searchQuery;

    this.adminService.listTenants(params).subscribe({
      next: (tenants) => {
        this.tenants.set(tenants);
        this.filteredTenants.set(tenants);
        this.loading.set(false);
      },
      error: (err) => {
        this.showError('Fehler beim Laden der Kunden');
        this.loading.set(false);
      },
    });
  }

  loadTenantDetails(id: string) {
    this.adminService.getTenant(id).subscribe({
      next: (tenant) => {
        this.selectedTenant.set(tenant);
        this.showDetailModal.set(true);
      },
      error: (err) => {
        this.showError('Fehler beim Laden der Kundendetails');
      },
    });
  }

  loadPlans() {
    this.adminService.listPlans().subscribe({
      next: (plans) => {
        this.plans.set(plans);
      },
      error: (err) => {
        this.showError('Fehler beim Laden der Pläne');
      },
    });
  }

  loadStatistics() {
    this.adminService.getStatistics().subscribe({
      next: (stats) => {
        this.statistics.set(stats);
      },
      error: (err) => {
        console.error('Fehler beim Laden der Statistiken', err);
      },
    });
  }

  // ============================================================================
  // FILTERING & SEARCH
  // ============================================================================

  onSearchChange() {
    this.currentPage = 1;
    this.loadTenants();
  }

  onFilterChange() {
    this.currentPage = 1;
    this.loadTenants();
  }

  clearFilters() {
    this.searchQuery = '';
    this.filterStatus = '';
    this.filterPlan = '';
    this.currentPage = 1;
    this.loadTenants();
  }

  // ============================================================================
  // TENANT CRUD
  // ============================================================================

  openCreateModal() {
    this.newTenant = {
      name: '',
      plan_id: '',
      status: 'trial',
      vat_number: '',
      tax_id: '',
      website: '',
      industry: '',
      company_size: '',
      preferred_language: 'de',
      notes: '',
    };
    this.showCreateModal.set(true);
  }

  createTenant() {
    if (!this.newTenant.name || !this.newTenant.plan_id) {
      this.showError('Name und Plan sind erforderlich');
      return;
    }

    this.loading.set(true);
    this.adminService.createTenant(this.newTenant).subscribe({
      next: (tenant) => {
        this.showSuccess('Kunde erfolgreich erstellt');
        this.showCreateModal.set(false);
        this.loadTenants();
        this.loadStatistics();
      },
      error: (err) => {
        this.showError(err.error?.message || 'Fehler beim Erstellen des Kunden');
        this.loading.set(false);
      },
    });
  }

  openEditModal() {
    const tenant = this.selectedTenant();
    if (!tenant) return;

    this.editTenant = {
      name: tenant.name,
      status: tenant.status || null,
      vat_number: tenant.vat_number || null,
      tax_id: tenant.tax_id || null,
      website: tenant.website || null,
      industry: tenant.industry || null,
      company_size: tenant.company_size || null,
      preferred_language: tenant.preferred_language || null,
      notes: tenant.notes || null,
      valid_until: tenant.valid_until || undefined,
    };
    this.showEditModal.set(true);
  }

  updateTenant() {
    const tenant = this.selectedTenant();
    if (!tenant) return;

    this.loading.set(true);
    this.adminService.updateTenant(tenant.id, this.editTenant).subscribe({
      next: (updatedTenant) => {
        this.showSuccess('Kunde erfolgreich aktualisiert');
        this.showEditModal.set(false);
        this.selectedTenant.set(updatedTenant);
        this.loadTenants();
      },
      error: (err) => {
        this.showError('Fehler beim Aktualisieren des Kunden');
        this.loading.set(false);
      },
    });
  }

  confirmDelete(tenant: TenantListItem) {
    this.tenantToDelete = tenant;
    this.showDeleteConfirm.set(true);
  }

  deleteTenant() {
    if (!this.tenantToDelete) return;

    this.loading.set(true);
    this.adminService.deleteTenant(this.tenantToDelete.id).subscribe({
      next: () => {
        this.showSuccess('Kunde erfolgreich gelöscht');
        this.showDeleteConfirm.set(false);
        this.showDetailModal.set(false);
        this.selectedTenant.set(null);
        this.tenantToDelete = null;
        this.loadTenants();
        this.loadStatistics();
      },
      error: (err) => {
        this.showError('Fehler beim Löschen des Kunden');
        this.loading.set(false);
      },
    });
  }

  // ============================================================================
  // STATUS & PLAN MANAGEMENT
  // ============================================================================

  updateStatus(tenantId: string, status: string) {
    this.loading.set(true);
    this.adminService.updateTenantStatus(tenantId, status).subscribe({
      next: (updatedTenant) => {
        this.showSuccess('Status erfolgreich geändert');
        if (this.selectedTenant()?.id === tenantId) {
          this.selectedTenant.set(updatedTenant);
        }
        this.loadTenants();
        this.loadStatistics();
      },
      error: (err) => {
        this.showError('Fehler beim Ändern des Status');
        this.loading.set(false);
      },
    });
  }

  openPlanModal() {
    const tenant = this.selectedTenant();
    if (!tenant) return;

    this.selectedPlanId = tenant.plan_id || '';
    this.showPlanModal.set(true);
  }

  switchPlan() {
    const tenant = this.selectedTenant();
    if (!tenant || !this.selectedPlanId) return;

    this.loading.set(true);
    this.adminService.switchTenantPlan(tenant.id, this.selectedPlanId).subscribe({
      next: (updatedTenant) => {
        this.showSuccess('Plan erfolgreich gewechselt');
        this.showPlanModal.set(false);
        this.selectedTenant.set(updatedTenant);
        this.loadTenants();
      },
      error: (err) => {
        this.showError('Fehler beim Wechseln des Plans');
        this.loading.set(false);
      },
    });
  }

  // ============================================================================
  // NAVIGATION
  // ============================================================================

  viewTenantDetails(tenant: TenantListItem) {
    this.loadTenantDetails(tenant.id);
  }

  navigateToAddresses(tenantId: string) {
    this.router.navigate(['/stax-admin/tenants', tenantId, 'addresses']);
  }

  navigateToContacts(tenantId: string) {
    this.router.navigate(['/stax-admin/tenants', tenantId, 'contacts']);
  }

  navigateToPayments(tenantId: string) {
    this.router.navigate(['/stax-admin/tenants', tenantId, 'payments']);
  }

  // ============================================================================
  // HELPER METHODS
  // ============================================================================

  getPlanName(planId: string | null | undefined): string {
    if (!planId) return '-';
    const plan = this.plans().find((p) => p.id === planId);
    return plan?.name || planId;
  }

  getStatusBadgeColor(status: string | null | undefined): string {
    return this.adminService.getStatusBadgeColor(status || '');
  }

  getStatusText(status: string | null | undefined): string {
    return this.adminService.getStatusText(status || '');
  }

  formatDate(dateString: string | null | undefined): string {
    return this.adminService.formatDate(dateString);
  }

  formatDateTime(dateString: string | null | undefined): string {
    return this.adminService.formatDateTime(dateString);
  }

  closeAllModals() {
    this.showCreateModal.set(false);
    this.showEditModal.set(false);
    this.showDeleteConfirm.set(false);
    this.showPlanModal.set(false);
    this.showDetailModal.set(false);
  }

  showError(message: string) {
    this.errorMessage.set(message);
    setTimeout(() => this.errorMessage.set(null), 5000);
  }

  showSuccess(message: string) {
    this.successMessage.set(message);
    setTimeout(() => this.successMessage.set(null), 3000);
  }

  // Pagination
  nextPage() {
    this.currentPage++;
    this.loadTenants();
  }

  previousPage() {
    if (this.currentPage > 1) {
      this.currentPage--;
      this.loadTenants();
    }
  }

  // Company sizes for dropdown
  companySizes = ['1-10', '11-50', '51-200', '201-500', '501+'];

  // Industries for dropdown
  industries = ['Healthcare', 'Emergency Services', 'Medical Services', 'Hospital', 'Other'];

  // Status options
  statusOptions = [
    { value: 'trial', label: 'Testphase' },
    { value: 'active', label: 'Aktiv' },
    { value: 'suspended', label: 'Gesperrt' },
    { value: 'canceled', label: 'Gekündigt' },
  ];
}
