# Instrucciones: Agente de Limpieza de Tests

**Objetivo**: Remover todos los tests existentes que puedan estar acoplados a implementación o que no sigan las mejores prácticas de testing comportamental.

**Contexto**: El proyecto Mega Brisas está refactorizando su suite de tests para enfocarse en testing de comportamiento (behavior-driven) en lugar de testing de implementación. Los tests actuales pueden contener:
- Mocks excesivos que acoplan a implementación
- Validaciones de detalles internos (queries SQL, orden de llamadas)
- Tests tautológicos que replican la lógica del código
- Tests frágiles que fallan ante refactors legítimos

---

## 1. Alcance de la Limpieza

### Archivos a Procesar

```
src-tauri/
├── src/
│   ├── contratista/
│   │   └── tests.rs         ← REMOVER completamente
│   ├── ingreso/
│   │   └── tests.rs         ← REMOVER completamente
│   ├── reportes/
│   │   └── tests.rs         ← REMOVER completamente
│   └── lib.rs
└── tests/                   ← REMOVER directorio completo
```

### Qué NO Remover

- **NO** remover el directorio `src-tauri/src/` (solo los archivos `tests.rs` dentro de cada módulo)
- **NO** remover archivos de configuración (`Cargo.toml`, `.github/workflows/`)
- **NO** remover código de producción (archivos `mod.rs`, `service.rs`, `repository.rs`)

---

## 2. Procedimiento de Ejecución

### Paso 1: Inventariar Tests Existentes

Antes de remover, crear un archivo de reporte con la lista de tests actuales:

```bash
# Buscar todos los tests existentes
grep -r "#\[test\]" src-tauri/src/ > /tmp/tests_inventario.txt
grep -r "#\[cfg(test)\]" src-tauri/src/ >> /tmp/tests_inventario.txt

# Contar total
echo "Total de tests encontrados: $(grep -c "#\[test\]" /tmp/tests_inventario.txt)"
```

**Guardar este reporte** en `/home/claude/tests_removed_report.md` con el formato:

```markdown
# Reporte de Tests Removidos
**Fecha**: 2026-01-08
**Agente**: Test Cleanup Agent

## Tests Eliminados por Módulo

### Módulo: contratista
- `test_crear_contratista_valido` (línea 45)
- `test_validar_cedula_duplicada` (línea 67)
Total: 2 tests

### Módulo: ingreso
- `test_registrar_ingreso` (línea 23)
Total: 1 test

**Gran Total: 3 tests removidos**
```

### Paso 2: Remover Módulos de Test

```bash
cd src-tauri

# Remover bloques #[cfg(test)] de cada módulo
for module in contratista ingreso reportes; do
    if [ -f "src/$module/tests.rs" ]; then
        echo "Removiendo src/$module/tests.rs"
        rm "src/$module/tests.rs"
    fi
    
    # Remover declaración de módulo de tests en mod.rs
    sed -i '/#\[cfg(test)\]/,/^$/d' "src/$module/mod.rs"
done

# Remover directorio de integration tests
if [ -d "tests" ]; then
    echo "Removiendo directorio tests/"
    rm -rf tests
fi
```

### Paso 3: Verificar Compilación

```bash
# El código de producción debe seguir compilando
cargo check

# Los tests ahora deben ser 0
cargo test --no-fail-fast 2>&1 | tee /tmp/test_cleanup_verification.txt
```

**Criterio de Éxito**: 
- `cargo check` pasa sin errores
- `cargo test` muestra: "running 0 tests"

---

## 3. Casos Especiales

### Si encuentras tests en archivos de producción

Algunos tests pueden estar embebidos directamente en archivos como `service.rs`:

```rust
// ❌ Este patrón debe removerse
impl ContratistaService {
    // código de producción
    
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_algo() { }
    }
}
```

**Acción**: Remover el bloque `#[cfg(test)] mod tests { ... }` completo.

### Si encuentras doc tests

```rust
/// Registra un nuevo contratista
/// 
/// ```
/// let service = ContratistaService::new();
/// let result = service.crear(contratista);
/// assert!(result.is_ok());
/// ```
pub fn crear(&self, contratista: Contractor) -> Result<()> { }
```

**Acción**: **NO remover** estos doc tests. Son parte de la documentación y se mantendrán hasta que se decida su futuro.

---

## 4. Validación Post-Limpieza

### Checklist de Verificación

```bash
# 1. No debe haber archivos tests.rs
find src-tauri/src -name "tests.rs" | wc -l
# Output esperado: 0

# 2. No debe haber bloques #[cfg(test)]
grep -r "#\[cfg(test)\]" src-tauri/src | wc -l
# Output esperado: 0 (o solo doc tests)

# 3. No debe haber directorio tests/
[ ! -d "src-tauri/tests" ] && echo "✓ Directorio tests/ removido"

# 4. El código debe compilar
cargo check && echo "✓ Código compila sin tests"

# 5. Test count debe ser 0
cargo test 2>&1 | grep "running 0 tests" && echo "✓ Suite de tests vacía"
```

---

## 5. Entregables

Al finalizar, debes generar:

1. **`/home/claude/tests_removed_report.md`**: Reporte detallado de qué se removió
2. **`/home/claude/cleanup_verification.log`**: Output de las verificaciones del checklist
3. **Confirmación**: Mensaje indicando que el workspace está limpio y listo para nuevos tests

---

## 6. Ejemplo de Output Esperado

```
$ cargo test
   Compiling mega-brisas v0.1.0 (/workspace/src-tauri)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.34s
     Running unittests src/lib.rs (target/debug/deps/mega_brisas-xxxxx)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 7. Notas Importantes

- **No hagas commits**: Solo remueve los archivos, el humano hará el commit
- **Preserva historia Git**: No uses `git rm`, solo `rm` normal
- **Sé conservador**: Si tienes duda sobre si un archivo es de test, pregunta antes de remover
- **Documentación primero**: Genera el reporte ANTES de remover para tener trazabilidad

---

## 8. Comando de Ejecución Rápida (Una sola línea)

```bash
cd src-tauri && \
find src -name "tests.rs" -type f -delete && \
rm -rf tests/ && \
cargo check && \
echo "✓ Limpieza completada. Running cargo test para verificar..." && \
cargo test
```

**Usar solo si estás 100% seguro de que no hay tests críticos que preservar.**
