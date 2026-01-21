import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-inventory-tablet',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="p-6 bg-gray-50 min-h-screen">
      <header class="flex justify-between items-end mb-8">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Inventur-Modus</h1>
          <p class="text-gray-500">Wache Süd - Hauptlager</p>
        </div>
        <div class="flex space-x-3">
          <button class="bg-white border border-gray-200 px-4 py-2 rounded-lg shadow-sm font-medium">
            Filter
          </button>
          <button class="bg-amber-500 text-white px-6 py-2 rounded-lg shadow-md font-bold">
            Inventur abschließen
          </button>
        </div>
      </header>

      <div class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="bg-gray-50 border-b border-gray-100">
              <th class="p-4 font-semibold text-gray-600">Artikel</th>
              <th class="p-4 font-semibold text-gray-600 text-center">Soll</th>
              <th class="p-4 font-semibold text-gray-600 text-center">Ist-Stand</th>
              <th class="p-4 font-semibold text-gray-600">Status</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-50">
            <tr class="hover:bg-amber-50/30 transition-colors">
              <td class="p-4">
                <div class="font-bold text-gray-800 text-lg">Venenverweilkanüle G20</div>
                <div class="text-sm text-gray-400">Schrank A - Fach 2</div>
              </td>
              <td class="p-4 text-center text-lg font-medium text-gray-500">50</td>
              <td class="p-4">
                <div class="flex items-center justify-center space-x-4">
                  <button class="w-12 h-12 rounded-full bg-gray-100 text-2xl flex items-center justify-center active:scale-95 transition">-</button>
                  <input type="number" value="48" class="w-20 text-center text-xl font-bold border-2 border-gray-200 rounded-xl py-2 focus:border-amber-500 outline-none">
                  <button class="w-12 h-12 rounded-full bg-gray-100 text-2xl flex items-center justify-center active:scale-95 transition">+</button>
                </div>
              </td>
              <td class="p-4 text-center">
                <span class="bg-green-100 text-green-700 px-3 py-1 rounded-full text-sm font-bold uppercase">Geprüft</span>
              </td>
            </tr>
            
            </tbody>
        </table>
      </div>

      <footer class="mt-8 bg-white p-4 rounded-xl shadow-inner border border-gray-100">
        <div class="flex justify-between mb-2 text-sm font-bold text-gray-600">
          <span>Fortschritt Inventur</span>
          <span>65% (32/50 Artikel)</span>
        </div>
        <div class="w-full bg-gray-200 h-3 rounded-full overflow-hidden">
          <div class="bg-amber-500 h-full w-[65%] transition-all duration-500"></div>
        </div>
      </footer>
    </div>
  `,
  styles: [`
    /* Verhindert die Standard-Pfeile im Number-Input für ein saubereres UI */
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
      -webkit-appearance: none;
      margin: 0;
    }
  `]
})
export class InvTabletComponent {}
