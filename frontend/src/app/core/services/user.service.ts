import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface User {
  id: string;
  username: string;
  first_name: string | null;
  last_name: string | null;
  role: string;
  is_active: boolean;
  tenant_id: string | null;
  created_at: string;
}

export interface UserDetail extends User {
  roles: UserRole[];
  permissions: Permission[];
}

export interface UserRole {
  role_id: string;
  role_name: string;
  display_name: string;
  assigned_at: string;
}

export interface Permission {
  permission_name: string;
  source: string;
}

export interface CreateUserRequest {
  username: string;
  password: string;
  first_name?: string;
  last_name?: string;
  role_ids?: string[];
  tenant_id?: string;
}

export interface UpdateUserRequest {
  first_name?: string;
  last_name?: string;
  is_active?: boolean;
}

export interface Role {
  id: string;
  name: string;
  display_name: string;
  description: string | null;
  is_system: boolean;
  permission_count: number;
}

export interface PermissionListItem {
  id: string;
  name: string;
  resource: string;
  action: string;
  scope: string;
  description: string | null;
}

@Injectable({
  providedIn: 'root',
})
export class UserService {
  private http = inject(HttpClient);
  private readonly API_URL = '/api/v1/users';

  // User CRUD
  listUsers(): Observable<User[]> {
    return this.http.get<User[]>(this.API_URL);
  }

  getUser(userId: string): Observable<UserDetail> {
    return this.http.get<UserDetail>(`${this.API_URL}/${userId}`);
  }

  createUser(user: CreateUserRequest): Observable<UserDetail> {
    return this.http.post<UserDetail>(this.API_URL, user);
  }

  updateUser(userId: string, updates: UpdateUserRequest): Observable<UserDetail> {
    return this.http.put<UserDetail>(`${this.API_URL}/${userId}`, updates);
  }

  deleteUser(userId: string): Observable<void> {
    return this.http.delete<void>(`${this.API_URL}/${userId}`);
  }

  // Password Management
  updatePassword(userId: string, newPassword: string): Observable<void> {
    return this.http.put<void>(`${this.API_URL}/${userId}/password`, {
      new_password: newPassword,
    });
  }

  // Role Management
  getUserRoles(userId: string): Observable<UserRole[]> {
    return this.http.get<UserRole[]>(`${this.API_URL}/${userId}/roles`);
  }

  assignRole(userId: string, roleId: string): Observable<void> {
    return this.http.post<void>(`${this.API_URL}/${userId}/roles`, {
      role_id: roleId,
    });
  }

  removeRole(userId: string, roleId: string): Observable<void> {
    return this.http.delete<void>(`${this.API_URL}/${userId}/roles/${roleId}`);
  }

  // Permission Management
  getUserPermissions(userId: string): Observable<Permission[]> {
    return this.http.get<Permission[]>(`${this.API_URL}/${userId}/permissions`);
  }

  assignPermission(
    userId: string,
    permissionName: string,
    expiresAt?: string
  ): Observable<void> {
    return this.http.post<void>(`${this.API_URL}/${userId}/permissions`, {
      permission_name: permissionName,
      expires_at: expiresAt,
    });
  }

  removePermission(userId: string, permissionId: string): Observable<void> {
    return this.http.delete<void>(
      `${this.API_URL}/${userId}/permissions/${permissionId}`
    );
  }

  // Available Roles & Permissions
  listAvailableRoles(): Observable<Role[]> {
    return this.http.get<Role[]>(`${this.API_URL}/roles/available`);
  }

  listAvailablePermissions(): Observable<PermissionListItem[]> {
    return this.http.get<PermissionListItem[]>(
      `${this.API_URL}/permissions/available`
    );
  }
}
