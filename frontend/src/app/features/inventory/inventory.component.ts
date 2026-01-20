import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-inventory',
  standalone: true,
  imports: [CommonModule],
  template: `<div class="p-8"><h1 class="text-3xl font-bold">StaxInventory</h1><p class="text-gray-600">In Entwicklung</p></div>`,
})
export class InventoryComponent {}
