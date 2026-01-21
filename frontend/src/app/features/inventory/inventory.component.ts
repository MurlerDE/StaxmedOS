import { Component, inject } from '@angular/core';
import { BreakpointObserver, Breakpoints } from '@angular/cdk/layout';
import { toSignal } from '@angular/core/rxjs-interop';
import { map } from 'rxjs';

// Import der drei Ansichten
import { InvMobileComponent } from './views/mobile/mobile.component';
import { InvTabletComponent } from './views/tablet/tablet.component';
import { InvDesktopComponent } from './views/desktop/desktop.component';

@Component({
  selector: 'app-inventory',
  standalone: true,
  imports: [InvMobileComponent, InvTabletComponent, InvDesktopComponent],
  template: `
    <div class="inventory-shell h-full w-full">
      @if (layout() === 'mobile') {
        <app-inventory-mobile />
      } @else if (layout() === 'tablet') {
        <app-inventory-tablet />
      } @else {
        <app-inventory-desktop />
      }
    </div>
  `
})
export class InventoryComponent {
  private breakpointObserver = inject(BreakpointObserver);

  // Das Signal 'layout' ändert sich automatisch, wenn das Fenster skaliert wird
  layout = toSignal(
    this.breakpointObserver.observe([
      Breakpoints.Handset, // Mobile
      Breakpoints.Tablet   // Tablet
    ]).pipe(
      map(result => {
        if (result.breakpoints[Breakpoints.Handset]) return 'mobile';
        if (result.breakpoints[Breakpoints.Tablet]) return 'tablet';
        return 'desktop';
      })
    ),
    { initialValue: 'desktop' }
  );
}
