import { Component } from '@angular/core';

@Component({
  selector: 'app-inventory-mobile',
  standalone: true,
  template: `
    <div class="p-4 bg-gray-50 min-h-screen">
      <header class="mb-6">
        <h2 class="text-xl font-bold">Schnellentnahme</h2>
        <p class="text-sm text-gray-500">Kamera scannen oder Artikel suchen</p>
      </header>

      <button
        class="w-full bg-amber-500 text-white py-8 rounded-2xl shadow-lg mb-4 flex flex-col items-center justify-center"
      >
        <span class="text-3xl mb-2">📸</span>
        <span class="font-bold">Barcode scannen</span>
      </button>

      <div class="space-y-2">
        <input
          type="text"
          placeholder="Artikel suchen..."
          class="w-full p-4 rounded-xl border-2 border-gray-200"
        />
      </div>
    </div>
  `,
})
export class InvMobileComponent {}
