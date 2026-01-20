import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { AuthService } from '../../core/auth/auth.service';

@Component({
  selector: 'app-landing-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './landing-page.component.html',
  styleUrl: './landing-page.component.css',
})
export class LandingPageComponent {
  auth = inject(AuthService);

  features = [
    {
      icon: '📦',
      title: 'StaxInventory',
      description:
        'Intelligente Lagerverwaltung mit automatischer Nachbestellung und Chargenverfolgung nach MPG.',
      highlights: ['Automatische Bestandsüberwachung', 'GS1-Scan Integration', 'Ablaufmanagement'],
    },
    {
      icon: '✅',
      title: 'StaxCompliance',
      description:
        'Automatische Dokumentation aller gesetzlichen Vorgaben für Medizinprodukte und Rettungsdienste.',
      highlights: ['MPG-konform', 'Digitale Prüfprotokolle', 'Audit-ready'],
    },
    {
      icon: '📅',
      title: 'StaxPlanner',
      description: 'Effiziente Dienst- und Schichtplanung mit automatischer Qualifikationsprüfung.',
      highlights: ['Qualifikations-Check', 'Urlaubsverwaltung', 'Mobil verfügbar'],
    },
    {
      icon: '🎓',
      title: 'StaxSkills',
      description:
        'Zentrale Verwaltung von Qualifikationen, Schulungen und Fortbildungen Ihres Teams.',
      highlights: ['Ablaufüberwachung', 'Erinnerungen', 'Zertifikatsverwaltung'],
    },
  ];

  benefits = [
    {
      stat: '70%',
      label: 'Zeitersparnis bei der Inventur',
      description: 'Durch automatisches Scannen und digitale Erfassung',
    },
    {
      stat: '100%',
      label: 'MPG-Konformität',
      description: 'Lückenlose Dokumentation aller Medizinprodukte',
    },
    {
      stat: '24/7',
      label: 'Verfügbarkeit',
      description: 'Cloud oder On-Premise – Sie entscheiden',
    },
  ];

  testimonials = [
    {
      quote:
        'StaxmedOS hat unsere Lagerverwaltung revolutioniert. Keine verpassten Ablaufdaten mehr!',
      author: 'Max Mustermann',
      role: 'Leiter Logistik',
      organization: 'Rettungswache Musterstadt',
    },
    {
      quote: 'Die automatische Compliance-Dokumentation spart uns unzählige Stunden pro Monat.',
      author: 'Dr. Sarah Schmidt',
      role: 'Ärztliche Leitung',
      organization: 'Notarzt Service Nord',
    },
  ];

  pricing = [
    {
      name: 'SaaS Cloud',
      price: '49€',
      period: 'pro Monat',
      description: 'Perfekt für kleine bis mittlere Wachen',
      features: [
        'Alle Module inklusive',
        'Unbegrenzte Benutzer',
        'Automatische Updates',
        'Cloud-Hosting',
        'E-Mail Support',
      ],
      highlighted: false,
    },
    {
      name: 'StaxBox',
      price: '299€',
      period: 'einmalig',
      description: 'Für maximale Datensicherheit',
      features: [
        'Alle Module inklusive',
        'Unbegrenzte Benutzer',
        'On-Premise Installation',
        'Ihre eigenen Server',
        'Premium Support',
        'Keine monatlichen Kosten',
      ],
      highlighted: true,
    },
  ];
}
