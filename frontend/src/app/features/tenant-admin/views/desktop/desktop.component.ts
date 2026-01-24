import { Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import {
  UserService,
  User,
  UserDetail,
  Role,
  CreateUserRequest,
} from '../../../../core/services/user.service';
import { AuthService } from '../../../../core/services/auth.service';

@Component({
  selector: 'app-tenant-admin-desktop',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './desktop.component.html',
  styleUrl: './desktop.component.css',
})
export class DesktopComponent implements OnInit {
  private userService = inject(UserService);
  auth = inject(AuthService);

  // State
  users = signal<User[]>([]);
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

  // Forms
  newUser: CreateUserRequest = {
    username: '',
    password: '',
    first_name: '',
    last_name: '',
    role_ids: [],
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
        // Filter system roles für normale Tenant Admins
        const filtered = roles.filter((r) => r.name !== 'staxadmin' || this.auth.isStaxAdmin());
        this.availableRoles.set(filtered);
      },
      error: (err) => {
        this.showError('Fehler beim Laden der Rollen');
      },
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
    };
    this.showCreateModal.set(true);
  }

  createUser() {
    if (!this.newUser.username || !this.newUser.password) {
      this.showError('Benutzername und Passwort sind erforderlich');
      return;
    }

    // Wenn keine Rollen ausgewählt, Standard-Paramedic-Rolle zuweisen
    if (!this.newUser.role_ids || this.newUser.role_ids.length === 0) {
      const paramedicRole = this.availableRoles().find((r) => r.name === 'paramedic');
      if (paramedicRole) {
        this.newUser.role_ids = [paramedicRole.id];
      }
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
        this.showError(
          err.error?.message ||
            'Fehler beim Erstellen des Benutzers. Möglicherweise existiert der Benutzername bereits.',
        );
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
  // Permission Helpers
  // ============================================================================

  canCreateUsers(): boolean {
    return this.auth.hasPermission('users.create') || this.auth.isAdmin();
  }

  canEditUsers(): boolean {
    return this.auth.hasPermission('users.update') || this.auth.isAdmin();
  }

  canDeleteUsers(): boolean {
    return this.auth.hasPermission('users.delete') || this.auth.isAdmin();
  }

  canManagePermissions(): boolean {
    return this.auth.hasPermission('users.manage_permissions') || this.auth.isAdmin();
  }
}
