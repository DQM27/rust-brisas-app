import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.join(__dirname, '..');

// Configuración de rutas
const PATHS = {
	package: path.join(rootDir, 'package.json'),
	tauri: path.join(rootDir, 'src-tauri', 'tauri.conf.json'),
	cargo: path.join(rootDir, 'src-tauri', 'Cargo.toml')
};

// Colores para la terminal
const green = (text) => `\x1b[32m${text}\x1b[0m`;
const red = (text) => `\x1b[31m${text}\x1b[0m`;
const cyan = (text) => `\x1b[36m${text}\x1b[0m`;
const yellow = (text) => `\x1b[33m${text}\x1b[0m`;

function sync() {
	try {
		// 1. Leer y validar package.json
		if (!fs.existsSync(PATHS.package)) throw new Error('No se encontró package.json');
		const pkg = JSON.parse(fs.readFileSync(PATHS.package, 'utf8'));
		const version = pkg.version;

		if (!/^\d+\.\d+\.\d+/.test(version)) {
			throw new Error(`Formato de versión inválido: ${version}. Use X.Y.Z`);
		}

		console.log(cyan(`🚀 Iniciando sincronización de versión: ${version}`));

		// 2. Actualizar tauri.conf.json
		if (fs.existsSync(PATHS.tauri)) {
			const tauriConf = JSON.parse(fs.readFileSync(PATHS.tauri, 'utf8'));
			tauriConf.version = version;
			fs.writeFileSync(PATHS.tauri, JSON.stringify(tauriConf, null, 2) + '\n');
			console.log(green('  ✅ src-tauri/tauri.conf.json actualizado'));
		}

		// 3. Actualizar Cargo.toml
		if (fs.existsSync(PATHS.cargo)) {
			let cargoToml = fs.readFileSync(PATHS.cargo, 'utf8');
			// Reemplaza la versión en la sección [package]
			const newCargoToml = cargoToml.replace(/^version = ".*"/m, `version = "${version}"`);

			if (cargoToml === newCargoToml) {
				throw new Error('No se pudo encontrar la línea de versión en src-tauri/Cargo.toml');
			}

			fs.writeFileSync(PATHS.cargo, newCargoToml);
			console.log(green('  ✅ src-tauri/Cargo.toml actualizado'));

			// 4. Forzar actualización de Cargo.lock (Muy importante para Rust)
			try {
				console.log(yellow('  ⏳ Sincronizando Cargo.lock (esto puede tardar unos segundos)...'));
				execSync('cargo fetch', { cwd: path.join(rootDir, 'src-tauri'), stdio: 'ignore' });
				console.log(green('  ✅ src-tauri/Cargo.lock sincronizado'));
			} catch (e) {
				console.log(
					yellow(
						'  ⚠️ No se pudo actualizar Cargo.lock (asegúrate de tener Rust instalado y estar conectado a internet)'
					)
				);
			}
		}

		console.log(cyan('\n✨ Proceso completado con éxito.'));
	} catch (error) {
		console.error(red(`\n❌ Error de sincronización: ${error.message}`));
		process.exit(1);
	}
}

sync();
