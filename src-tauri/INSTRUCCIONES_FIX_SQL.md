# Guía Rápida: Ejecutar Fix SQL

## Paso 1: Ubicar la Base de Datos

Busca el archivo de base de datos en tu proyecto. Debería estar en:
- `src-tauri/*.db`
- O en el directorio de datos de la app

## Paso 2: Abrir con SQLite

### Opción A: DB Browser for SQLite (Recomendado)
1. Descarga: https://sqlitebrowser.org/dl/
2. Abre DB Browser
3. File → Open Database
4. Selecciona tu archivo `.db`
5. Ve a la pestaña "Execute SQL"
6. Pega el SQL de abajo
7. Click en el botón "Play" (▶️)

### Opción B: Línea de Comandos
```powershell
# Navega al directorio
cd src-tauri

# Abre SQLite
sqlite3 <nombre_de_tu_db>.db

# Ejecuta el SQL
INSERT OR IGNORE INTO users VALUES (
  '00000000-0000-0000-0000-000000000000',
  'system@brisas.local', 
  '$2b$12$placeholder',
  'Sistema', 
  'Temporal', 
  'guardia', 
  1,
  datetime('now'), 
  datetime('now')
);

# Verifica
SELECT id, email, nombre FROM users WHERE id = '00000000-0000-0000-0000-000000000000';

# Sal
.exit
```

## Paso 3: Verificar

Ejecuta en la consola SQL:
```sql
SELECT id, email, nombre, apellido FROM users 
WHERE id = '00000000-0000-0000-0000-000000000000';
```

Deberías ver:
```
id: 00000000-0000-0000-0000-000000000000
email: system@brisas.local
nombre: Sistema
apellido: Temporal
```

## Paso 4: Recargar App

1. Recarga la aplicación
2. Intenta registrar un ingreso
3. Debería funcionar ✅
