<!-- src/lib/components/layout/sidebar/panels/SettingsPanel.svelte -->
<script lang="ts">
	import {
		Settings,
		Bell,
		Download,
		RefreshCw,
		Info,
		FileText,
		Home,
		Shield,
		Palette,
		Clock
	} from '@lucide/svelte';
	import { openView, activePanel } from '../../../../stores/sidebar';

	function handleKeydown(e: KeyboardEvent, action: () => void) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			action();
		}
	}

	function executeAndClose(action: () => void) {
		return () => {
			action();
			activePanel.set(null);
		};
	}
</script>

<div class="panel-section">
	<div class="panel-section-title">CONFIGURACIÓN GENERAL</div>
	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('general-settings', 'Ajustes Generales'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('general-settings', 'Ajustes Generales'))
			)}
	>
		<Settings size={16} />
		<span>Ajustes Generales</span>
	</button>

	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('visual-settings', 'Ajustes Gráficos'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('visual-settings', 'Ajustes Gráficos'))
			)}
	>
		<Palette size={16} />
		<span>Ajustes Gráficos</span>
	</button>

	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('export-settings', 'Configuración de Exportación'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('export-settings', 'Configuración de Exportación'))
			)}
	>
		<FileText size={16} />
		<span>Exportación</span>
	</button>

	<div class="panel-item non-clickable">
		<Bell size={16} />
		<span>Notificaciones</span>
	</div>
	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('backup-settings', 'Copias de Seguridad'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('backup-settings', 'Copias de Seguridad'))
			)}
	>
		<Download size={16} />
		<span>Backup y restore</span>
	</button>
	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('roles-settings', 'Roles y Permisos'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('roles-settings', 'Roles y Permisos'))
			)}
	>
		<Shield size={16} />
		<span>Roles y Permisos</span>
	</button>
</div>

<div class="panel-section">
	<div class="panel-section-title">SISTEMA</div>
	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('welcome', 'Bienvenida'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('welcome', 'Bienvenida'))
			)}
	>
		<Home size={16} />
		<span>Pantalla de Bienvenida</span>
	</button>
	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('session-settings', 'Gestión de Sesión'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('session-settings', 'Gestión de Sesión'))
			)}
	>
		<Clock size={16} />
		<span>Gestión de Sesión</span>
	</button>
	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('update-settings', 'Actualizaciones'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('update-settings', 'Actualizaciones'))
			)}
	>
		<RefreshCw size={16} />
		<span>Actualizaciones</span>
	</button>
	<button
		class="panel-item"
		onclick={executeAndClose(() => openView('about', 'Acerca del Sistema'))}
		onkeydown={(e) =>
			handleKeydown(
				e,
				executeAndClose(() => openView('about', 'Acerca del Sistema'))
			)}
	>
		<Info size={16} />
		<span>Acerca del sistema</span>
	</button>
</div>
