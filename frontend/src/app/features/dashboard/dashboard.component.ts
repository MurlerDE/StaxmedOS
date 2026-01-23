import { Component, inject, signal, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink } from '@angular/router';
import { AuthService } from '../../core/services/auth.service';

interface StatCard {
  title: string;
  value: string | number;
  change: string;
  changeType: 'positive' | 'negative' | 'neutral';
  icon: string;
  trend: number[];
}

interface Activity {
  id: string;
  type: 'delivery' | 'alert' | 'maintenance' | 'order' | 'info';
  title: string;
  description: string;
  time: string;
  status: 'completed' | 'pending' | 'warning' | 'info';
  user?: string;
}

interface Alert {
  id: string;
  severity: 'high' | 'medium' | 'low';
  title: string;
  description: string;
  count?: number;
  location?: string;
}

interface QuickAction {
  id: string;
  title: string;
  icon: string;
  route: string;
  description: string;
  color: string;
}

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [CommonModule, RouterLink],
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.css',
})
export class DashboardComponent implements OnInit {
  auth = inject(AuthService);

  // Current time
  currentTime = signal(new Date());
  greeting = signal('');

  // Stats
  stats = signal<StatCard[]>([
    {
      title: 'Gesamtbestand',
      value: '2.847',
      change: '+12% zum Vormonat',
      changeType: 'positive',
      icon: '📦',
      trend: [100, 120, 115, 140, 135, 160, 155, 180],
    },
    {
      title: 'Kritische Artikel',
      value: 23,
      change: '8 ablaufend diese Woche',
      changeType: 'negative',
      icon: '⚠️',
      trend: [50, 45, 48, 42, 40, 35, 30, 23],
    },
    {
      title: 'Offene Aufgaben',
      value: 7,
      change: '4 heute fällig',
      changeType: 'neutral',
      icon: '✅',
      trend: [15, 12, 14, 10, 9, 8, 7, 7],
    },
    {
      title: 'Fahrzeuge einsatzbereit',
      value: '5/6',
      change: 'RTW-3 in Wartung',
      changeType: 'neutral',
      icon: '🚑',
      trend: [6, 6, 5, 6, 6, 5, 5, 5],
    },
  ]);

  // Alerts
  alerts = signal<Alert[]>([
    {
      id: '1',
      severity: 'high',
      title: 'Mindestbestand unterschritten',
      description: 'Infusionslösungen (500ml NaCl)',
      count: 3,
      location: 'Zentrallager',
    },
    {
      id: '2',
      severity: 'high',
      title: 'Ablauf in 7 Tagen',
      description: 'Adrenalin-Ampullen, Charge XY-2024',
      count: 12,
    },
    {
      id: '3',
      severity: 'medium',
      title: 'MTK-Prüfung fällig',
      description: 'Defibrillator RTW-2',
      location: 'RTW-2',
    },
    {
      id: '4',
      severity: 'low',
      title: 'Bestellung ausstehend',
      description: 'Einmalhandschuhe Größe L',
      count: 5,
    },
  ]);

  // Recent Activities
  activities = signal<Activity[]>([
    {
      id: '1',
      type: 'delivery',
      title: 'Neue Lieferung eingegangen',
      description: '147 Artikel gescannt und eingelagert',
      time: 'vor 2 Stunden',
      status: 'completed',
      user: 'M. Schmidt',
    },
    {
      id: '2',
      type: 'alert',
      title: 'Mindestbestand erreicht',
      description: 'Beatmungsmasken Größe 3',
      time: 'vor 3 Stunden',
      status: 'warning',
    },
    {
      id: '3',
      type: 'maintenance',
      title: 'MTK-Prüfung durchgeführt',
      description: 'Defibrillator RTW-1 - bestanden',
      time: 'vor 5 Stunden',
      status: 'completed',
      user: 'J. Müller',
    },
    {
      id: '4',
      type: 'order',
      title: 'Bestellung versendet',
      description: 'Nachbestellung Verbrauchsmaterial',
      time: 'Gestern',
      status: 'info',
      user: 'S. Weber',
    },
    {
      id: '5',
      type: 'info',
      title: 'Inventur abgeschlossen',
      description: 'RTW-4 - keine Abweichungen',
      time: 'Gestern',
      status: 'completed',
      user: 'T. Fischer',
    },
  ]);

  // Quick Actions
  quickActions = signal<QuickAction[]>([
    {
      id: '1',
      title: 'Inventar scannen',
      icon: '📱',
      route: '/inventory',
      description: 'Artikel erfassen',
      color: 'bg-blue-500',
    },
    {
      id: '2',
      title: 'Lieferung prüfen',
      icon: '📋',
      route: '/inventory',
      description: 'Wareneingang',
      color: 'bg-green-500',
    },
    {
      id: '3',
      title: 'Bestellung aufgeben',
      icon: '🛒',
      route: '/inventory',
      description: 'Material nachbestellen',
      color: 'bg-amber-500',
    },
    {
      id: '4',
      title: 'Wartung melden',
      icon: '🔧',
      route: '/compliance',
      description: 'Gerät zur Prüfung',
      color: 'bg-red-500',
    },
    {
      id: '5',
      title: 'Dienst eintragen',
      icon: '📅',
      route: '/planner',
      description: 'Schichtplan',
      color: 'bg-purple-500',
    },
    {
      id: '6',
      title: 'Schulung buchen',
      icon: '🎓',
      route: '/skills',
      description: 'Fortbildung',
      color: 'bg-indigo-500',
    },
  ]);

  // Upcoming Events
  upcomingEvents = signal([
    {
      id: '1',
      title: 'Großübung Massenanfall',
      date: 'Morgen, 10:00 Uhr',
      type: 'training',
      participants: 12,
    },
    {
      id: '2',
      title: 'Teammeeting Logistik',
      date: 'Freitag, 14:00 Uhr',
      type: 'meeting',
      participants: 6,
    },
    {
      id: '3',
      title: 'STK-Prüfung Fahrzeuge',
      date: 'Nächste Woche',
      type: 'maintenance',
      participants: 3,
    },
  ]);

  ngOnInit() {
    this.updateGreeting();
    // Update time every minute
    setInterval(() => {
      this.currentTime.set(new Date());
      this.updateGreeting();
    }, 60000);
  }

  private updateGreeting() {
    const hour = new Date().getHours();
    if (hour < 12) {
      this.greeting.set('Guten Morgen');
    } else if (hour < 18) {
      this.greeting.set('Guten Tag');
    } else {
      this.greeting.set('Guten Abend');
    }
  }

  getAlertColor(severity: string): string {
    switch (severity) {
      case 'high':
        return 'bg-red-100 border-red-300 text-red-800';
      case 'medium':
        return 'bg-orange-100 border-orange-300 text-orange-800';
      case 'low':
        return 'bg-yellow-100 border-yellow-300 text-yellow-800';
      default:
        return 'bg-gray-100 border-gray-300 text-gray-800';
    }
  }

  getAlertIcon(severity: string): string {
    switch (severity) {
      case 'high':
        return '🔴';
      case 'medium':
        return '🟠';
      case 'low':
        return '🟡';
      default:
        return 'ℹ️';
    }
  }

  getActivityIcon(type: string): string {
    switch (type) {
      case 'delivery':
        return '📦';
      case 'alert':
        return '⚠️';
      case 'maintenance':
        return '🔧';
      case 'order':
        return '🛒';
      case 'info':
        return 'ℹ️';
      default:
        return '📋';
    }
  }

  getActivityColor(status: string): string {
    switch (status) {
      case 'completed':
        return 'bg-green-100 text-green-800';
      case 'pending':
        return 'bg-blue-100 text-blue-800';
      case 'warning':
        return 'bg-orange-100 text-orange-800';
      case 'info':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  }

  getStatusBadge(status: string): string {
    switch (status) {
      case 'completed':
        return 'Abgeschlossen';
      case 'pending':
        return 'In Bearbeitung';
      case 'warning':
        return 'Warnung';
      case 'info':
        return 'Info';
      default:
        return status;
    }
  }
}
