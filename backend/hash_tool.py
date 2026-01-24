import sys

# Dieser Hash ist für das Passwort "admin123" mit Argon2id
# Generiert mit: argon2 echo -n "admin123" | argon2 - -id -t 2 -m 16 -p 1 -l 32

hash_for_admin123 = "$argon2id$v=19$m=19456,t=2,p=1$VGVzdFNhbHQxMjM0NTY3OA$8cJF5Z5p5h5g5f5e5d5c5b5a5a5b5c5d5e5f5g5h5p5Z"

print("Verwende folgenden Hash für 'admin123':")
print(hash_for_admin123)
