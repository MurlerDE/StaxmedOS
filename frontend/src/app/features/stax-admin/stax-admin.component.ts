import { Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import {
  UserService,
  User,
  UserDetail,
  Role,
  CreateUserRequest,
} from '../../core/services/user.service';
import { AuthService } from '../../core/services/auth.service';
import { TenantsComponent } from './tenants/tenants.component';

@Component({
  selector: 'app-stax-admin',
  standalone: true,
  imports: [CommonModule, FormsModule, TenantsComponent],
  templateUrl: './stax-admin.component.html',
  styleUrl: './stax-admin.component.css',
})
export class StaxAdminComponent implements OnInit {
  private userService = inject(UserService);
  auth = inject(AuthService);

  // Active Tab
  activeTab: 'tenants' | 'users' = 'tenants';

  // State
  users = signal<User[]>([]);
  filteredUsers = signal<User[]>([]);
  selectedUser = signal<UserDetail | null>(null);
  availableRoles = signal<Role[]>([]);
  loading = signal(false);
  showCreateModal = signal(false);
  showEditModal = signal(false);
  showRoleModal = signal(false);
  showPasswordModal = signal(false);
  showDeleteConfirm = signal(false);
  errorMessage = signal<string | null>(null);
  successMessage = signal<string | null>(null);

  // Filters (StaxAdmin-specific)
  filterTenant = '';
  filterRole = '';
  filterStatus = 'all'; // all, active, inactive
  searchQuery = '';
  showAllUsers = false; // Default: nur Admins anzeigen

  // Forms
  newUser: CreateUserRequest = {
    username: '',
    password: '',
    first_name: '',
    last_name: '',
    role_ids: [],
    tenant_id: '',
  };

  editUser = {
    first_name: '',
    last_name: '',
    is_active: true,
  };

  newPassword = '';
  confirmPassword = '';
  userToDelete: User | null = null;
  selectedRoleId = '';

  // Statistics
  stats = signal({
    total: 0,
    active: 0,
    inactive: 0,
    admins: 0,
    staxadmins: 0,
  });

  ngOnInit() {
    this.loadUsers();
    this.loadAvailableRoles();
  }

  // ============================================================================
  // Data Loading
  // ============================================================================

  loadUsers() {
    this.loading.set(true);
    this.userService.listUsers().subscribe({
      next: (users) => {
        this.users.set(users);
        this.applyFilters();
        this.loading.set(false);
      },
      error: (err) => {
        this.showError('Fehler beim Laden der Benutzer');
        this.loading.set(false);
      },
    });
  }

  loadUserDetails(userId: string) {
    this.userService.getUser(userId).subscribe({
      next: (user) => {
        this.selectedUser.set(user);
      },
      error: (err) => {
        this.showError('Fehler beim Laden der Benutzerdetails');
      },
    });
  }

  loadAvailableRoles() {
    this.userService.listAvailableRoles().subscribe({
      next: (roles) => {
        this.availableRoles.set(roles);
      },
      error: (err) => {
        this.showError('Fehler beim Laden der Rollen');
      },
    });
  }

  // ============================================================================
  // Filtering & Search
  // ============================================================================

  applyFilters() {
    let filtered = [...this.users()];

    // Default: Nur Admins anzeigen (außer "Alle anzeigen" ist aktiv)
    if (!this.showAllUsers) {
      filtered = filtered.filter((u) => u.role === 'admin' || u.role === 'staxadmin');
    }

    // Search
    if (this.searchQuery) {
      const query = this.searchQuery.toLowerCase();
      filtered = filtered.filter(
        (u) =>
          u.username.toLowerCase().includes(query) ||
          u.first_name?.toLowerCase().includes(query) ||
          u.last_name?.toLowerCase().includes(query),
      );
    }

    // Role filter
    if (this.filterRole) {
      filtered = filtered.filter((u) => u.role === this.filterRole);
    }

    // Status filter
    if (this.filterStatus === 'active') {
      filtered = filtered.filter((u) => u.is_active);
    } else if (this.filterStatus === 'inactive') {
      filtered = filtered.filter((u) => !u.is_active);
    }

    // Tenant filter
    if (this.filterTenant) {
      filtered = filtered.filter((u) => u.tenant_id === this.filterTenant);
    }

    this.filteredUsers.set(filtered);
    this.calculateStats(filtered);
  }

  onSearchChange() {
    this.applyFilters();
  }

  onFilterChange() {
    this.applyFilters();
  }

  clearFilters() {
    this.searchQuery = '';
    this.filterRole = '';
    this.filterStatus = 'all';
    this.filterTenant = '';
    this.showAllUsers = false; // Zurück zu nur Admins
    this.applyFilters();
  }

  toggleShowAllUsers() {
    this.showAllUsers = !this.showAllUsers;
    this.applyFilters();
  }

  calculateStats(users: User[]) {
    // Statistiken basieren auf gefilterter Ansicht
    const visibleUsers = this.filteredUsers();
    this.stats.set({
      total: visibleUsers.length,
      active: visibleUsers.filter((u) => u.is_active).length,
      inactive: visibleUsers.filter((u) => !u.is_active).length,
      admins: visibleUsers.filter((u) => u.role === 'admin').length,
      staxadmins: visibleUsers.filter((u) => u.role === 'staxadmin').length,
    });
  }

  // ============================================================================
  // User Selection & Details
  // ============================================================================

  selectUser(user: User) {
    this.loadUserDetails(user.id);
  }

  clearSelection() {
    this.selectedUser.set(null);
  }

  // ============================================================================
  // Create User
  // ============================================================================

  openCreateModal() {
    this.newUser = {
      username: '',
      password: '',
      first_name: '',
      last_name: '',
      role_ids: [],
      tenant_id: '',
    };
    this.showCreateModal.set(true);
  }

  createUser() {
    if (!this.newUser.username || !this.newUser.password) {
      this.showError('Benutzername und Passwort sind erforderlich');
      return;
    }

    if (!this.newUser.tenant_id) {
      this.showError('Tenant ist erforderlich');
      return;
    }

    this.loading.set(true);
    this.userService.createUser(this.newUser).subscribe({
      next: (user) => {
        this.showSuccess('Benutzer erfolgreich erstellt');
        this.showCreateModal.set(false);
        this.loadUsers();
        this.selectUser(user);
      },
      error: (err) => {
        this.showError(err.error?.message || 'Fehler beim Erstellen des Benutzers');
        this.loading.set(false);
      },
    });
  }

  // ============================================================================
  // Update User
  // ============================================================================

  openEditModal() {
    const user = this.selectedUser();
    if (!user) return;

    this.editUser = {
      first_name: user.first_name || '',
      last_name: user.last_name || '',
      is_active: user.is_active,
    };
    this.showEditModal.set(true);
  }

  updateUser() {
    const user = this.selectedUser();
    if (!user) return;

    this.loading.set(true);
    this.userService.updateUser(user.id, this.editUser).subscribe({
      next: (updatedUser) => {
        this.showSuccess('Benutzer erfolgreich aktualisiert');
        this.showEditModal.set(false);
        this.selectedUser.set(updatedUser);
        this.loadUsers();
      },
      error: (err) => {
        this.showError('Fehler beim Aktualisieren des Benutzers');
        this.loading.set(false);
      },
    });
  }

  // ============================================================================
  // Delete User
  // ============================================================================

  confirmDelete(user: User) {
    this.userToDelete = user;
    this.showDeleteConfirm.set(true);
  }

  deleteUser() {
    if (!this.userToDelete) return;

    this.loading.set(true);
    this.userService.deleteUser(this.userToDelete.id).subscribe({
      next: () => {
        this.showSuccess('Benutzer erfolgreich gelöscht');
        this.showDeleteConfirm.set(false);
        this.clearSelection();
        this.loadUsers();
        this.userToDelete = null;
      },
      error: (err) => {
        this.showError('Fehler beim Löschen des Benutzers');
        this.loading.set(false);
      },
    });
  }

  // ============================================================================
  // Password Management
  // ============================================================================

  openPasswordModal() {
    this.newPassword = '';
    this.confirmPassword = '';
    this.showPasswordModal.set(true);
  }

  updatePassword() {
    const user = this.selectedUser();
    if (!user) return;

    if (this.newPassword !== this.confirmPassword) {
      this.showError('Passwörter stimmen nicht überein');
      return;
    }

    if (this.newPassword.length < 8) {
      this.showError('Passwort muss mindestens 8 Zeichen lang sein');
      return;
    }

    this.loading.set(true);
    this.userService.updatePassword(user.id, this.newPassword).subscribe({
      next: () => {
        this.showSuccess('Passwort erfolgreich geändert');
        this.showPasswordModal.set(false);
        this.newPassword = '';
        this.confirmPassword = '';
        this.loading.set(false);
      },
      error: (err) => {
        this.showError('Fehler beim Ändern des Passworts');
        this.loading.set(false);
      },
    });
  }

  // ============================================================================
  // Role Management
  // ============================================================================

  openRoleModal() {
    this.selectedRoleId = '';
    this.showRoleModal.set(true);
  }

  assignRole() {
    const user = this.selectedUser();
    if (!user || !this.selectedRoleId) return;

    this.loading.set(true);
    this.userService.assignRole(user.id, this.selectedRoleId).subscribe({
      next: () => {
        this.showSuccess('Rolle erfolgreich zugewiesen');
        this.showRoleModal.set(false);
        this.loadUserDetails(user.id);
        this.loading.set(false);
      },
      error: (err) => {
        this.showError('Fehler beim Zuweisen der Rolle');
        this.loading.set(false);
      },
    });
  }

  removeRole(roleId: string) {
    const user = this.selectedUser();
    if (!user) return;

    if (!confirm('Möchten Sie diese Rolle wirklich entfernen?')) return;

    this.loading.set(true);
    this.userService.removeRole(user.id, roleId).subscribe({
      next: () => {
        this.showSuccess('Rolle erfolgreich entfernt');
        this.loadUserDetails(user.id);
      },
      error: (err) => {
        this.showError('Fehler beim Entfernen der Rolle');
        this.loading.set(false);
      },
    });
  }

  // ============================================================================
  // Helper Functions
  // ============================================================================

  getDisplayName(user: User): string {
    if (user.first_name && user.last_name) {
      return `${user.first_name} ${user.last_name}`;
    }
    if (user.first_name) return user.first_name;
    if (user.last_name) return user.last_name;
    return user.username;
  }

  getInitials(user: User): string {
    if (user.first_name && user.last_name) {
      return `${user.first_name[0]}${user.last_name[0]}`.toUpperCase();
    }
    if (user.first_name) return user.first_name[0].toUpperCase();
    return user.username[0].toUpperCase();
  }

  getRoleBadgeColor(roleName: string): string {
    switch (roleName) {
      case 'staxadmin':
        return 'bg-red-600';
      case 'admin':
        return 'bg-blue-600';
      case 'logistic_manager':
        return 'bg-green-600';
      case 'paramedic':
        return 'bg-gray-600';
      default:
        return 'bg-purple-600';
    }
  }

  formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString('de-DE', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  closeAllModals() {
    this.showCreateModal.set(false);
    this.showEditModal.set(false);
    this.showRoleModal.set(false);
    this.showPasswordModal.set(false);
    this.showDeleteConfirm.set(false);
  }

  showError(message: string) {
    this.errorMessage.set(message);
    setTimeout(() => this.errorMessage.set(null), 5000);
  }

  showSuccess(message: string) {
    this.successMessage.set(message);
    setTimeout(() => this.successMessage.set(null), 3000);
  }

  // ============================================================================
  // Unique Tenants for Filter
  // ============================================================================

  getUniqueTenants(): { id: string; name: string }[] {
    const tenantMap = new Map<string, string>();
    this.users().forEach((user) => {
      if (user.tenant_id) {
        tenantMap.set(user.tenant_id, user.tenant_id);
      }
    });
    return Array.from(tenantMap.entries()).map(([id]) => ({ id, name: id }));
  }
}
