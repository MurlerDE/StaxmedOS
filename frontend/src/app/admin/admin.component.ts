import { Component, inject, signal, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-admin',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './admin.component.html',
})
export class AdminComponent implements OnInit {
  private http = inject(HttpClient);
  tenants = signal<any[]>([]);

  ngOnInit() {
    this.loadTenants();
  }

  loadTenants() {
    this.http
      .get<any[]>('http://127.0.0.1:3000/api/admin/tenants')
      .subscribe((data) => this.tenants.set(data));
  }
}
