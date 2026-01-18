import { Component, inject } from '@angular/core';
import { AuthService } from '../../services/auth.service';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [CommonModule],
  template: `
    @if (auth.isLoggedIn()) {
      <div class="p-8">
        <h1 class="text-3xl font-bold">Willkommen zurück!</h1>
        <p class="text-gray-600 mt-2">Hier ist die Übersicht deines Mandanten.</p>
        <div class="grid grid-cols-3 gap-6 mt-8">
          <div class="bg-white p-6 rounded-xl shadow-sm border">Statistik A</div>
          <div class="bg-white p-6 rounded-xl shadow-sm border">Statistik B</div>
          <div class="bg-white p-6 rounded-xl shadow-sm border">Statistik C</div>
        </div>
      </div>
    } @else {
      <div class="flex flex-col items-center justify-center py-20 px-4 text-center">
        <h1 class="text-5xl font-extrabold text-gray-900 mb-6">Medizinlogistik neu gedacht.</h1>
        <p class="text-xl text-gray-600 max-w-2xl mb-10">
          StaxmedOS ist das Betriebssystem für deine Einrichtung. Von der Lagerhaltung bis zur
          Personalplanung – alles in einer Hand.
        </p>
        <button
          (click)="auth.showLoginModal.set(true)"
          class="bg-amber-500 text-white px-8 py-4 rounded-full text-lg font-bold shadow-lg hover:bg-amber-600 transition"
        >
          Jetzt kostenlos testen
        </button>
      </div>
    }
  `,
})
export class HomeComponent {
  auth = inject(AuthService);
}
