#!/bin/bash

# Generate a secure JWT secret for StaxmedOS
# Usage: ./generate-jwt-secret.sh

echo "Generating secure JWT secret..."
echo ""

# Check if openssl is available
if command -v openssl &> /dev/null; then
    SECRET=$(openssl rand -base64 32)
    echo "Generated JWT Secret:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "$SECRET"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Add this to your .env file:"
    echo "JWT_SECRET=$SECRET"
else
    echo "❌ OpenSSL not found. Please install OpenSSL or generate a secret manually."
    echo ""
    echo "Alternative: Use a random string generator or run this in Node.js:"
    echo "node -e \"console.log(require('crypto').randomBytes(32).toString('base64'))\""
fi

echo ""
echo "⚠️  IMPORTANT: Keep this secret safe and never commit it to Git!"
