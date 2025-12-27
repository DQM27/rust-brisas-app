# SurrealDB Integration (Experimental)

> **Rama:** `experiment/surrealdb`  
> **Fecha:** 2025-12-26  
> **Estado:** Usuarios funcionando, Roles pendiente

## Resumen

Se integró SurrealDB como base de datos embebida alternativa a SQLite, usando RocksDB para persistencia local. El objetivo es tener una DB más moderna con soporte para relaciones, LIVE queries, y tiempo real.

## Archivos Creados/Modificados

### Backend (`src-tauri/src/`)

| Archivo | Descripción |
|---------|-------------|
| `services/surrealdb_service.rs` | Servicio de conexión embebida (RocksDB) |
| `services/surrealdb_user_service.rs` | Servicio de usuarios para SurrealDB |
| `db/surrealdb_user_queries.rs` | Queries idiomáticas (FETCH, MERGE, type::thing) |
| `db/surrealdb_schema.surql` | Esquema simplificado (sin SCHEMAFULL) |
| `config/surrealdb_seed.rs` | Seeds para roles y usuarios |
| `commands/user_commands.rs` | Comandos condicionales SQLite/SurrealDB |

### Configuración

| Archivo | Cambio |
|---------|--------|
| `Cargo.toml` | Feature `surrealdb-backend` con `kv-rocksdb` |
| `tauri.surrealdb.conf.json` | Config separada para SurrealDB |
| `package.json` | Script `npm run tauri:surrealdb` |

## Cómo Ejecutar

```bash
# Con SQLite (normal)
npm run tauri dev

# Con SurrealDB (experimental)
npm run tauri:surrealdb
```

## Ubicación de los Datos

| Backend | Ruta |
|---------|------|
| SQLite Producción | `~/.local/share/Brisas/brisas.db` |
| SQLite Demo | `~/.local/share/Brisas/brisas_demo.db` |
| SurrealDB Producción | `~/.local/share/Brisas/surrealdb/` |
| SurrealDB Demo | `~/.local/share/Brisas/surrealdb_demo/` |

## Feature Flag

El feature `surrealdb-backend` activa:
1. Inicialización de SurrealDB embebido en `lib.rs`
2. Seeds automáticos de roles y usuarios
3. Comandos de usuario usan SurrealDB en vez de SQLite

```rust
#[cfg(feature = "surrealdb-backend")]
// Código específico de SurrealDB
```

## Lo que Funciona ✅

- Login con SurrealDB
- Verificación de password (usa keyring secret de Argon2)
- Cambio de contraseña
- Get user by ID
- Get all users
- Create user
- Update user
- Delete user (soft delete)
- Seeds automáticos (roles, superuser, admin)

## Lo que Falta ❌

- **Módulo de Roles/Permisos** - aún usa SQLite
  - `get_user_visible_modules()` consulta `role_permissions` de SQLite
  - Por eso no aparecen módulos Users/Config aunque seas admin
- **Otras tablas** - contratistas, ingresos, etc.
- **LIVE Queries** - notificaciones en tiempo real

## Problemas Resueltos

1. **Schema SCHEMAFULL** causaba que CREATE fallara silenciosamente
   - Solución: Schema simplificado sin tipos estrictos

2. **ID con brackets** `⟨uuid⟩` causaba NotFound
   - Solución: Usar `to_raw()` en vez de `to_string()`

3. **Password verification** fallaba
   - Solución: Usar `auth::verify_password()` con keyring secret

4. **Seeds ejecutaban antes del keyring**
   - Los passwords se hashean con secret del keyring, que debe estar configurado

## Próximos Pasos

1. Migrar módulo de Roles/Permisos a SurrealDB
2. Implementar `get_user_visible_modules()` para SurrealDB
3. Migrar otras tablas gradualmente
4. Implementar LIVE queries para notificaciones

## Notas Importantes

- **Primera compilación de RocksDB tarda ~5-10 min** (después es rápido)
- **La PC del trabajo (i3)** puede compilar sin `--features surrealdb-backend` normalmente
- Si hay problemas de login, borrar `~/.local/share/Brisas/surrealdb*/` y reiniciar
