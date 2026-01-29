import { Injectable, signal, effect } from '@angular/core';

export type AppSchema = 'clinical' | 'operator' | 'tactical';
export type AppTheme = 'light' | 'dark';

@Injectable({ providedIn: 'root' })
export class ThemeService {
  // 1. Signals für den aktuellen Zustand
  // Wir versuchen zuerst aus dem LocalStorage zu laden, sonst Default
  schema = signal<AppSchema>(
    (localStorage.getItem('stax-schema') as AppSchema) || 'operator'
  );
  
  theme = signal<AppTheme>(
    (localStorage.getItem('stax-theme') as AppTheme) || 'light'
  );

  constructor() {
    // 2. Der Magic-Effect: Läuft automatisch, wenn sich ein Signal ändert
    effect(() => {
      const currentSchema = this.schema();
      const currentTheme = this.theme();

      // Attribute am HTML-Tag setzen (für die CSS-Selektoren)
      document.documentElement.setAttribute('data-schema', currentSchema);
      document.documentElement.setAttribute('data-theme', currentTheme);

      // In LocalStorage persistieren
      localStorage.setItem('stax-schema', currentSchema);
      localStorage.setItem('stax-theme', currentTheme);
    });
  }

  // Hilfsmethoden zum Umschalten
  setSchema(newSchema: AppSchema) {
    this.schema.set(newSchema);
  }

  toggleTheme() {
    this.theme.update(t => t === 'light' ? 'dark' : 'light');
  }
}
