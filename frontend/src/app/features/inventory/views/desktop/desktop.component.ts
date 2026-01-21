@Component({
  selector: 'app-inventory-desktop',
  standalone: true,
  template: `
    <div class="p-8">
      <div class="flex justify-between items-center mb-8">
        <h1 class="text-3xl font-black text-gray-800">Lagerverwaltung</h1>
        <button class="bg-amber-500 text-white px-6 py-2 rounded-lg font-bold">+ Neuer Artikel</button>
      </div>
      
      <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
        <p class="text-gray-400 italic">Lade Bestandsliste...</p>
      </div>
    </div>
  `
})
export class InvDesktopComponent {}
