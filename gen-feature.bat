@echo off
REM Nutzung: gen-feature.bat name-des-features

setlocal EnableDelayedExpansion

if "%~1"=="" (
    echo Bitte einen Feature-Namen angeben z.B. gen-feature.bat compliance
    exit /b 1
)

set FEATURE_NAME=%~1
set BASE_PATH=frontend\src\app\features\%FEATURE_NAME%

echo.
echo ======================================
echo   Erstelle Feature: %FEATURE_NAME%
echo ======================================
echo.

REM 1. Ordnerstruktur erstellen
echo [1/4] Erstelle Ordnerstruktur...
if not exist "%BASE_PATH%\components" mkdir "%BASE_PATH%\components"
if not exist "%BASE_PATH%\services" mkdir "%BASE_PATH%\services"
if not exist "%BASE_PATH%\views\mobile" mkdir "%BASE_PATH%\views\mobile"
if not exist "%BASE_PATH%\views\tablet" mkdir "%BASE_PATH%\views\tablet"
if not exist "%BASE_PATH%\views\desktop" mkdir "%BASE_PATH%\views\desktop"
echo    Ordner erstellt: %BASE_PATH%

REM 2. Angular CLI nutzen für die Grund-Komponenten (Standalone)
echo.
echo [2/4] Erstelle Haupt-Komponente...
cd frontend
call ng g c "features/%FEATURE_NAME%" --standalone --inline-template --inline-style --skip-tests
if errorlevel 1 (
    echo FEHLER: Konnte Haupt-Komponente nicht erstellen
    cd ..
    exit /b 1
)

echo.
echo [3/4] Erstelle View-Komponenten...
call ng g c "features/%FEATURE_NAME%/views/mobile" --standalone --skip-tests
call ng g c "features/%FEATURE_NAME%/views/tablet" --standalone --skip-tests
call ng g c "features/%FEATURE_NAME%/views/desktop" --standalone --skip-tests

cd ..

REM 3. Routing Datei erstellen
echo.
echo [4/4] Erstelle Routing-Datei...

REM Ersten Buchstaben groß schreiben für PascalCase
set FIRST_CHAR=%FEATURE_NAME:~0,1%
set REST_CHARS=%FEATURE_NAME:~1%

REM PowerShell nutzen für ToUpper (funktioniert zuverlässiger)
for /f %%i in ('powershell -command "'%FIRST_CHAR%'.ToUpper()"') do set FIRST_CHAR_UPPER=%%i
for /f %%i in ('powershell -command "'%FEATURE_NAME%'.ToUpper()"') do set FEATURE_NAME_UPPER=%%i

set COMPONENT_NAME=%FIRST_CHAR_UPPER%%REST_CHARS%

(
echo import { Routes } from '@angular/router';
echo import { %COMPONENT_NAME%Component } from './%FEATURE_NAME%.component';
echo.
echo export const %FEATURE_NAME_UPPER%_ROUTES: Routes = [
echo   { path: '', component: %COMPONENT_NAME%Component }
echo ];
) > "%BASE_PATH%\%FEATURE_NAME%.routes.ts"

echo    Routing-Datei erstellt: %FEATURE_NAME%.routes.ts

REM 4. Zusammenfassung
echo.
echo ======================================
echo   Feature erfolgreich erstellt!
echo ======================================
echo.
echo Erstellte Dateien:
echo   - %BASE_PATH%\%FEATURE_NAME%.component.ts
echo   - %BASE_PATH%\views\mobile\mobile.component.ts
echo   - %BASE_PATH%\views\tablet\tablet.component.ts
echo   - %BASE_PATH%\views\desktop\desktop.component.ts
echo   - %BASE_PATH%\%FEATURE_NAME%.routes.ts
echo.
echo Ordner:
echo   - %BASE_PATH%\components
echo   - %BASE_PATH%\services
echo   - %BASE_PATH%\views\mobile
echo   - %BASE_PATH%\views\tablet
echo   - %BASE_PATH%\views\desktop
echo.
echo Naechste Schritte:
echo   1. Routes in app.routes.ts einbinden
echo   2. Services in %BASE_PATH%\services erstellen
echo   3. Komponenten in %BASE_PATH%\components entwickeln
echo.

endlocal
