import { Injectable, signal, inject } from '@angular/core';
import { AuthService } from './auth.service';

interface IModal {
  id: string;
  element: HTMLDialogElement;
}

export type ModalType = 'login' | 'custom';

@Injectable({
  providedIn: 'root',
})
export class ModalService {
  private modals = signal<IModal[]>([]);

  // Login-Modal spezifische Signals
  showLoginModal = signal(false);
  loginLoading = signal(false);
  loginError = signal('');

  register(id: string, element: HTMLDialogElement) {
    this.modals.set([
      ...this.modals(),
      {
        id,
        element,
      },
    ]);
  }

  unregister(id: string) {
    this.modals.set(this.modals().filter((element) => element.id !== id));
  }

  toggle(id: string) {
    const modal = this.modals().find((item) => item.id === id);
    if (!modal) return;
    if (modal.element.open) {
      modal.element.close();
    } else {
      modal.element.showModal();
    }
  }

  open(id: string) {
    const modal = this.modals().find((item) => item.id === id);
    if (!modal) return;
    modal.element.showModal();
  }

  close(id: string) {
    const modal = this.modals().find((item) => item.id === id);
    if (!modal) return;
    modal.element.close();
  }

  // Login-Modal Helpers
  openLoginModal() {
    this.showLoginModal.set(true);
    this.loginError.set('');
    this.open('login-modal');
  }

  closeLoginModal() {
    this.showLoginModal.set(false);
    this.loginError.set('');
    this.close('login-modal');
  }

  setLoginError(error: string) {
    this.loginError.set(error);
    this.loginLoading.set(false);
  }

  setLoginLoading(loading: boolean) {
    this.loginLoading.set(loading);
  }
}
