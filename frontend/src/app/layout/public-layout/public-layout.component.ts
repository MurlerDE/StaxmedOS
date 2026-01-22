import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet, RouterLink } from '@angular/router';
import { AuthService } from '../../core/auth/auth.service';
import { ModalService } from '../../core/services/modal.service';
import { LoginModalComponent } from '../../features/auth/components/login-modal.component';

@Component({
  selector: 'public-layout',
  standalone: true,
  imports: [CommonModule, RouterOutlet, RouterLink, LoginModalComponent],
  templateUrl: './public-layout.component.html',
  styleUrl: './public-layout.component.css',
})
export class PublicLayoutComponent {
  auth = inject(AuthService);
  modalService = inject(ModalService);
}
