import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet, RouterLink } from '@angular/router';
import { AuthService } from '../../core/services/auth.service';
import { ModalService } from '../../core/services/modal.service';
import { AuthModalComponent } from '../../features/auth-modal/auth-modal.component';

@Component({
  selector: 'public-layout',
  standalone: true,
  imports: [CommonModule, RouterOutlet, RouterLink, AuthModalComponent],
  templateUrl: './public-layout.component.html',
  styleUrl: './public-layout.component.css',
})
export class PublicLayoutComponent {
  auth = inject(AuthService);
  modalService = inject(ModalService);

  openModal($event: Event) {
    $event.preventDefault();
    this.modalService.toggle('auth');
  }
}
