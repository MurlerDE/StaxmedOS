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
  `,
})
export class InventoryComponent {
  private breakpointObserver = inject(BreakpointObserver);

  // Das Signal 'layout' ändert sich automatisch, wenn das Fenster skaliert wird
  layout = toSignal(
    this.breakpointObserver
      .observe([
        Breakpoints.XSmall, // < 600px (Mobile Portrait)
        Breakpoints.Small, // 600px - 959px (Mobile Landscape, Tablet Portrait)
        Breakpoints.Medium, // 960px - 1279px (Tablet Landscape)
        Breakpoints.Large, // 1280px - 1919px (Desktop)
        Breakpoints.XLarge, // >= 1920px (Large Desktop)
      ])
      .pipe(
        map((result) => {
          // Die Reihenfolge ist wichtig: Vom kleinsten zum größten prüfen
          if (result.breakpoints[Breakpoints.XSmall]) return 'mobile';
          if (result.breakpoints[Breakpoints.Small]) return 'mobile';
          if (result.breakpoints[Breakpoints.Medium]) return 'tablet';
          // Large und XLarge = Desktop
          return 'desktop';
        }),
      ),
    { initialValue: 'desktop' },
  );
}
