import { Component, inject, signal, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-stax-admin',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './stax-admin.component.html',
})
export class StaxAdminComponent implements OnInit {
  private http = inject(HttpClient);
  tenants = signal<any[]>([]);

  ngOnInit() {
    this.loadTenants();
  }

  loadTenants() {
    this.http.get<any[]>('/api/v1/admin/tenants').subscribe((data) => this.tenants.set(data));
  }
}
