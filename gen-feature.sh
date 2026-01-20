#!/bin/bash
# Nutzung: ./gen-feature.sh name-des-features

FEATURE_NAME=$1

if [ -z "$FEATURE_NAME" ]; then
  echo "Bitte einen Feature-Namen angeben (z.B. ./gen-feature.sh compliance)"
  exit 1
fi

BASE_PATH="frontend/src/app/features/$FEATURE_NAME"

# 1. Ordnerstruktur erstellen
mkdir -p "$BASE_PATH/components"
mkdir -p "$BASE_PATH/services"
mkdir -p "$BASE_PATH/views/inv-mobile"
mkdir -p "$BASE_PATH/views/inv-tablet"
mkdir -p "$BASE_PATH/views/inv-desktop"

# 2. Angular CLI nutzen für die Grund-Komponenten (Standalone)
# Wir nutzen --skip-tests für den Anfang, falls du sie nicht sofort brauchst
ng g c "features/$FEATURE_NAME" --standalone --inline-template --inline-style
ng g c "features/$FEATURE_NAME/views/inv-mobile" --standalone
ng g c "features/$FEATURE_NAME/views/inv-tablet" --standalone
ng g c "features/$FEATURE_NAME/views/inv-desktop" --standalone

# 3. Routing Datei erstellen
cat <<EOF > "$BASE_PATH/$FEATURE_NAME-routing.ts"
import { Routes } from '@angular/router';
import { ${FEATURE_NAME^}Component } from './$FEATURE_NAME.component';

export const ${FEATURE_NAME^^}_ROUTES: Routes = [
  { path: '', component: ${FEATURE_NAME^}Component }
];
EOF

echo "✅ Feature $FEATURE_NAME wurde erfolgreich erstellt!"
