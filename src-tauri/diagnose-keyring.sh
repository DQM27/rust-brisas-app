#!/bin/bash
# Script de diagnóstico para problemas de keyring

echo "==================================="
echo "   DIAGNÓSTICO DE KEYRING"
echo "==================================="
echo ""

# 1. Verificar sistema operativo
echo "1. Sistema Operativo:"
echo "   OS: $(uname -s)"
echo "   Versión: $(uname -r)"
echo "   Desktop: $XDG_CURRENT_DESKTOP"
echo ""

# 2. Verificar servicios de keyring
echo "2. Servicios de Keyring:"
if ps aux | grep -q "[g]nome-keyring-daemon"; then
    echo "   ✓ gnome-keyring-daemon está corriendo"
    ps aux | grep "[g]nome-keyring-daemon" | head -1
else
    echo "   ✗ gnome-keyring-daemon NO está corriendo"
fi

if ps aux | grep -q "[k]wallet"; then
    echo "   ✓ kwallet está corriendo"
else
    echo "   ✗ kwallet NO está corriendo"
fi
echo ""

# 3. Verificar herramientas de keyring instaladas
echo "3. Herramientas de Keyring:"
for tool in gnome-keyring-daemon seahorse secret-tool kwallet5; do
    if which $tool &> /dev/null; then
        echo "   ✓ $tool: $(which $tool)"
    else
        echo "   ✗ $tool: no instalado"
    fi
done
echo ""

# 4. Verificar paquetes D-Bus
echo "4. D-Bus:"
if [ -n "$DBUS_SESSION_BUS_ADDRESS" ]; then
    echo "   ✓ DBUS_SESSION_BUS_ADDRESS: $DBUS_SESSION_BUS_ADDRESS"
else
    echo "   ✗ DBUS_SESSION_BUS_ADDRESS no está configurado"
fi
echo ""

# 5. Verificar servicio de secretos
echo "5. Secret Service API:"
if which secret-tool &> /dev/null; then
    echo "   Probando secret-tool..."
    if secret-tool store --label="test-brisas" application "brisas-diagnostic" key "test" <<< "test-value" 2>/dev/null; then
        echo "   ✓ Puede escribir secretos"

        VALUE=$(secret-tool lookup application "brisas-diagnostic" key "test" 2>/dev/null)
        if [ "$VALUE" = "test-value" ]; then
            echo "   ✓ Puede leer secretos correctamente"
        else
            echo "   ✗ Error leyendo secretos (esperado: 'test-value', obtenido: '$VALUE')"
        fi

        if secret-tool clear application "brisas-diagnostic" key "test" 2>/dev/null; then
            echo "   ✓ Puede eliminar secretos"
        else
            echo "   ⚠ No pudo eliminar el secreto de prueba"
        fi
    else
        echo "   ✗ NO puede escribir secretos"
    fi
else
    echo "   ⚠ secret-tool no está instalado"
fi
echo ""

# 6. Verificar dependencias de Rust
echo "6. Versión de Rust:"
rustc --version
echo ""

# 7. Verificar keyring en Cargo
echo "7. Dependencia keyring en Cargo:"
cd src-tauri 2>/dev/null || cd .
grep "keyring" Cargo.toml 2>/dev/null || echo "   No se encontró Cargo.toml en el directorio actual"
echo ""

# 8. Sugerencias
echo "==================================="
echo "   SUGERENCIAS"
echo "==================================="
echo ""

if ! ps aux | grep -q "[g]nome-keyring-daemon"; then
    echo "⚠ gnome-keyring-daemon no está corriendo."
    echo "  Solución: Reinicia tu sesión o ejecuta:"
    echo "  eval \$(gnome-keyring-daemon --start --components=secrets)"
    echo ""
fi

if ! which secret-tool &> /dev/null; then
    echo "⚠ secret-tool no está instalado."
    echo "  Solución (Ubuntu/Debian):"
    echo "  sudo apt install libsecret-tools"
    echo ""
    echo "  Solución (Fedora):"
    echo "  sudo dnf install libsecret"
    echo ""
fi

if [ -z "$DBUS_SESSION_BUS_ADDRESS" ]; then
    echo "⚠ DBUS_SESSION_BUS_ADDRESS no está configurado."
    echo "  Esto puede causar problemas con el keyring."
    echo "  Asegúrate de estar ejecutando la app desde tu sesión de escritorio."
    echo ""
fi

echo "==================================="
echo "Para probar la aplicación, ejecuta:"
echo "  npm run tauri dev"
echo ""
echo "Y abre el archivo test-keyring.html en el navegador integrado."
echo "==================================="
