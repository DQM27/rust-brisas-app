#!/bin/bash
echo "=== TEST DE PERSISTENCIA DE CREDENCIALES ==="
echo ""
echo "Paso 1: Limpiar credenciales anteriores..."
cd src-tauri
for key in smtp_host smtp_user; do
    secret-tool clear service brisas-app username "$key" 2>/dev/null
done

echo "Paso 2: Ejecutar test que guarda credenciales..."
cargo run --bin test_real 2>&1 | grep -E "(✓|✗|Probando)"

echo ""
echo "Paso 3: Verificar si persisten con secret-tool..."
echo "  Buscando smtp_host..."
VALUE=$(secret-tool lookup service brisas-app username smtp_host 2>/dev/null)
if [ -n "$VALUE" ]; then
    echo "  ✓ smtp_host encontrado: $VALUE"
else
    echo "  ✗ smtp_host NO encontrado"
fi

echo "  Buscando smtp_user..."
VALUE=$(secret-tool lookup service brisas-app username smtp_user 2>/dev/null)
if [ -n "$VALUE" ]; then
    echo "  ✓ smtp_user encontrado: $VALUE"
else
    echo "  ✗ smtp_user NO encontrado"
fi

echo ""
echo "Paso 4: Ejecutar el test NUEVAMENTE (debe leer las credenciales guardadas)..."
cargo run --bin test_real 2>&1 | grep -E "(✓|✗|Probando)"

echo ""
echo "=== FIN DEL TEST ==="
