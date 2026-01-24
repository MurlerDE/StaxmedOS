@echo off
echo Testing StaxmedOS Backend Connection...
echo.

echo Testing health endpoint...
curl -s http://localhost:3000/api/v1/healthy
echo.
echo.

echo If you see "StaxmedOS API v1 - Online" above, the backend is running!
echo.
echo If you see an error, start the backend with: start-backend.bat
echo.

pause
