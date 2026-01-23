import { inject } from '@angular/core';
import { Router, CanActivateFn } from '@angular/router';
import { AuthService } from '../services/auth.service';
import { ModalService } from '../services/modal.service';

export const authGuard: CanActivateFn = (route, state) => {
  const authService = inject(AuthService);
  const modalService = inject(ModalService);
  const router = inject(Router);

  if (authService.isLoggedIn()) {
    return true;
  }

  // User ist nicht eingeloggt, zeige Login-Modal
  modalService.openLoginModal();
  return false;
};

export const adminGuard: CanActivateFn = (route, state) => {
  const authService = inject(AuthService);
  const router = inject(Router);

  if (authService.isLoggedIn() && authService.isAdmin()) {
    return true;
  }

  // Nicht autorisiert - zurück zur Startseite
  router.navigate(['/']);
  return false;
};

export const publicGuard: CanActivateFn = (route, state) => {
  const authService = inject(AuthService);
  const router = inject(Router);

  // Wenn User eingeloggt ist, redirect zum Dashboard
  if (authService.isLoggedIn()) {
    router.navigate(['/dashboard']);
    return false;
  }

  // Nicht eingeloggt, Landing Page anzeigen
  return true;
};
