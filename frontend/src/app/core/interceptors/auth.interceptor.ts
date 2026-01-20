import { HttpInterceptorFn } from '@angular/common/http';

export const authInterceptor: HttpInterceptorFn = (req, next) => {
  // Direkt aus localStorage lesen um zirkuläre Abhängigkeit zu vermeiden
  // NICHT AuthService injizieren, da dieser den HttpClient benutzt!
  const token = localStorage.getItem('stax_token');

  // Wir hängen den Token nur an, wenn er existiert
  if (token) {
    const cloned = req.clone({
      setHeaders: {
        Authorization: `Bearer ${token}`,
      },
    });
    return next(cloned);
  }

  return next(req);
};
