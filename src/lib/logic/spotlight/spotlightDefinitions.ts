/**
 * Spotlight Search Definitions
 * Constantes de módulos, acciones y mapeos de componentes
 */

import {
	UserCircle2,
	DoorOpen,
	PackageCheck,
	Contact,
	IdCard,
	ShieldX,
	Plus,
	Settings,
	ArrowRight,
	Monitor,
	ShieldCheck,
	Users,
	Download,
	Database,
	RefreshCw,
	UserPlus,
	Ban,
	ListPlus,
	Search,
	History,
	Moon,
	Sun,
	LogOut,
	Keyboard,
	AlertTriangle,
	Home
} from '@lucide/svelte';
import type { SpotlightItemDefinition } from '$lib/types/spotlight';

// ============================================
// MODULE DEFINITIONS
// ============================================

export const MODULE_DEFINITIONS: SpotlightItemDefinition[] = [
	{
		id: 'users-list',
		label: 'Lista Usuarios',
		description: 'Gestión de usuarios del sistema',
		icon: UserCircle2,
		category: 'module',
		subCategory: 'link',
		keywords: ['user', 'usuarios', 'lista', 'administrar'],
		permission: 'users:view',
		shortcut: 'Shift+U'
	},
	{
		id: 'ingreso-list',
		label: 'Ingresos Contratistas',
		description: 'Control de ingresos de contratistas',
		icon: DoorOpen,
		category: 'module',
		subCategory: 'link',
		keywords: ['ingreso', 'contratista', 'entrada', 'acceso'],
		permission: 'ingresos:view',
		shortcut: 'Shift+I'
	},
	{
		id: 'contratista-list',
		label: 'Lista Contratistas',
		description: 'Gestión de catálogo de contratistas',
		icon: Users,
		category: 'module',
		subCategory: 'link',
		keywords: ['lista', 'contratista', 'maestro', 'administrar'],
		permission: 'contratistas:view',
		shortcut: 'Shift+C'
	},
	{
		id: 'proveedor-ingreso-list',
		label: 'Ingresos Proveedores',
		description: 'Control de ingresos de proveedores',
		icon: PackageCheck,
		category: 'module',
		subCategory: 'link',
		keywords: ['proveedor', 'ingreso', 'suministro', 'entrada'],
		permission: 'ingresos:view'
	},
	{
		id: 'proveedor-list',
		label: 'Lista Proveedores',
		description: 'Gestión de catálogo de proveedores',
		icon: PackageCheck,
		category: 'module',
		subCategory: 'link',
		keywords: ['lista', 'proveedor', 'maestro', 'administrar'],
		permission: 'proveedores:view'
	},
	{
		id: 'visitas-list',
		label: 'Ingresos Visitas',
		description: 'Control de ingresos de visitantes',
		icon: Contact,
		category: 'module',
		subCategory: 'link',
		keywords: ['visita', 'visitante', 'invitado', 'entrada'],
		permission: 'visitantes:view'
	},
	{
		id: 'visitante-list',
		label: 'Lista Visitantes',
		description: 'Catálogo histórico de visitantes',
		icon: Users,
		category: 'module',
		subCategory: 'link',
		keywords: ['lista', 'visitante', 'maestro', 'administrar'],
		permission: 'visitantes:view'
	},
	{
		id: 'gafete-list',
		label: 'Lista Gafetes',
		description: 'Gestión de inventario de gafetes',
		icon: IdCard,
		category: 'module',
		subCategory: 'link',
		keywords: ['gafete', 'identificacion', 'tarjeta', 'badge'],
		permission: 'gafetes:view'
	},
	{
		id: 'lista-negra-list',
		label: 'Lista Negra',
		description: 'Control de acceso restringido',
		icon: ShieldX,
		category: 'module',
		subCategory: 'link',
		keywords: ['lista negra', 'blacklist', 'bloqueado', 'restringido'],
		permission: 'lista_negra:view'
	},
	{
		id: 'alerta-gafete-list',
		label: 'Control de Alertas',
		description: 'Resolución de incidencias de gafetes',
		icon: AlertTriangle,
		category: 'module',
		subCategory: 'link',
		keywords: ['alerta', 'incidencia', 'resolucion', 'gafete', 'seguridad', 'pendiente'],
		permission: 'gafetes:view',
		shortcut: 'Alt+A'
	},
	{
		id: 'welcome',
		label: 'Bienvenida',
		description: 'Pantalla de inicio y resumen visual',
		icon: Home,
		category: 'module',
		subCategory: 'link',
		keywords: ['inicio', 'bienvenida', 'home', 'pantalla', 'principal'],
		shortcut: 'Shift+H'
	}
];

// ============================================
// ACTION DEFINITIONS
// ============================================

export const ACTION_DEFINITIONS: SpotlightItemDefinition[] = [
	// --- TRANSACCIONES (NUEVO) ---
	{
		id: 'create-contratista',
		label: 'Nuevo Ingreso Contratista',
		description: 'Registrar entrada de contratista',
		icon: Plus,
		category: 'action',
		subCategory: 'transaction',
		keywords: ['nuevo', 'crear', 'contratista', 'ingreso', 'registrar'],
		permission: 'ingresos:create',
		shortcut: 'Ctrl+N'
	},
	{
		id: 'create-proveedor',
		label: 'Nuevo Ingreso Proveedor',
		description: 'Registrar entrada de proveedor',
		icon: Plus,
		category: 'action',
		subCategory: 'transaction',
		keywords: ['nuevo', 'crear', 'proveedor', 'ingreso', 'registrar'],
		permission: 'ingresos:create'
	},
	{
		id: 'create-visita',
		label: 'Nuevo Ingreso Visita',
		description: 'Registrar entrada de visitante',
		icon: Plus,
		category: 'action',
		subCategory: 'transaction',
		keywords: ['nuevo', 'crear', 'visita', 'visitante', 'ingreso', 'registrar'],
		permission: 'visitantes:create'
	},
	// --- MAESTROS (CREAR) ---
	{
		id: 'master-contratista',
		label: 'Crear Contratista',
		description: 'Añadir nuevo contratista al catálogo',
		icon: UserPlus,
		category: 'action',
		subCategory: 'master',
		keywords: ['crear', 'maestro', 'catálogo', 'contratista'],
		permission: 'contratistas:create'
	},
	{
		id: 'master-proveedor',
		label: 'Crear Proveedor',
		description: 'Añadir nuevo proveedor al catálogo',
		icon: UserPlus,
		category: 'action',
		subCategory: 'master',
		keywords: ['crear', 'maestro', 'catálogo', 'proveedor'],
		permission: 'proveedores:create'
	},
	{
		id: 'master-visitante',
		label: 'Crear Visita',
		description: 'Añadir nuevo visitante al catálogo',
		icon: UserPlus,
		category: 'action',
		subCategory: 'master',
		keywords: ['crear', 'maestro', 'catálogo', 'visitante', 'visita'],
		permission: 'visitantes:create'
	},
	{
		id: 'create-user',
		label: 'Crear Usuario',
		description: 'Registrar un nuevo acceso al sistema',
		icon: UserPlus,
		category: 'action',
		subCategory: 'master',
		keywords: ['nuevo', 'crear', 'usuario', 'user', 'registrar', 'empleado'],
		permission: 'users:create'
	},
	{
		id: 'create-blacklist',
		label: 'Crear Bloqueo',
		description: 'Añadir persona a lista negra',
		icon: Ban,
		category: 'action',
		subCategory: 'master',
		keywords: ['bloquear', 'banear', 'lista negra', 'denegar', 'restringir'],
		permission: 'lista_negra:create'
	},
	{
		id: 'create-gafete',
		label: 'Crear Gafete',
		description: 'Registrar un nuevo gafete físico',
		icon: Plus,
		category: 'action',
		subCategory: 'master',
		keywords: ['nuevo', 'crear', 'gafete', 'carnet', 'tarjeta', 'inventario'],
		permission: 'gafetes:create'
	},
	{
		id: 'create-gafete-batch',
		label: 'Crear Lote de Gafetes',
		description: 'Generar múltiples gafetes por rango',
		icon: ListPlus,
		category: 'action',
		subCategory: 'master',
		keywords: ['lote', 'rango', 'generar', 'masivo', 'gafetes'],
		permission: 'gafetes:create'
	},
	// --- CONFIGURACIÓN ---
	{
		id: 'settings-general',
		label: 'Ajustes Generales',
		description: 'Preferencias globales del sistema',
		icon: Settings,
		category: 'action',
		subCategory: 'settings',
		keywords: ['configuracion', 'ajustes', 'general', 'sistema'],
		permission: 'settings_general:view'
	},
	{
		id: 'settings-visual',
		label: 'Ajustes Gráficos',
		description: 'Configuración visual y temas',
		icon: Monitor,
		category: 'action',
		subCategory: 'settings',
		keywords: ['configuracion', 'ajustes', 'graficos', 'visual', 'tema', 'modo oscuro'],
		permission: 'settings_visual:view'
	},
	{
		id: 'settings-session',
		label: 'Gestión de Sesión',
		description: 'Configuración de seguridad y sesiones',
		icon: ShieldCheck,
		category: 'action',
		subCategory: 'settings',
		keywords: ['configuracion', 'ajustes', 'sesion', 'seguridad', 'password'],
		permission: 'settings_sessions:view'
	},
	{
		id: 'settings-roles',
		label: 'Roles y Permisos',
		description: 'Gestión de roles de usuario',
		icon: Users,
		category: 'action',
		subCategory: 'settings',
		keywords: ['configuracion', 'roles', 'permisos', 'usuarios', 'acceso'],
		permission: 'roles:view'
	},
	{
		id: 'settings-export',
		label: 'Configuración de Exportación',
		description: 'Ajustes de reportes y exportación',
		icon: Download,
		category: 'action',
		subCategory: 'settings',
		keywords: ['configuracion', 'exportacion', 'reportes', 'excel', 'pdf'],
		permission: 'backup:view'
	},
	{
		id: 'settings-backup',
		label: 'Copias de Seguridad',
		description: 'Gestión de respaldos del sistema',
		icon: Database,
		category: 'action',
		subCategory: 'settings',
		keywords: ['configuracion', 'backup', 'respaldo', 'copia', 'seguridad'],
		permission: 'backup:view'
	},
	{
		id: 'settings-spotlight',
		label: 'Ajustes del Buscador',
		description: 'Personalizar experiencia de Spotlight',
		icon: Search,
		category: 'action',
		subCategory: 'settings',
		keywords: ['configuracion', 'ajustes', 'spotlight', 'buscador', 'omnibox'],
		permission: 'settings_general:view'
	},
	{
		id: 'settings-shortcuts',
		label: 'Configurar Atajos',
		description: 'Personalizar atajos de teclado',
		icon: Keyboard,
		category: 'action',
		subCategory: 'settings',
		keywords: ['atajos', 'teclado', 'shortcuts', 'teclas', 'personalizar', 'configurar']
	},
	{
		id: 'action-reindex',
		label: 'Reindexar Búsqueda',
		description: 'Regenerar índices de búsqueda',
		icon: RefreshCw,
		category: 'action',
		subCategory: 'link',
		keywords: ['reindexar', 'busqueda', 'indice', 'regenerar']
	},
	// --- SISTEMA (AJUSTES RÁPIDOS) ---
	{
		id: 'toggle-theme',
		label: 'Cambiar Tema (Oscuro/Claro)',
		description: 'Alternar entre tema visual oscuro y claro',
		icon: Moon,
		category: 'action',
		subCategory: 'settings',
		keywords: ['tema', 'oscuro', 'claro', 'diseño', 'color'],
		shortcut: 'Ctrl+T'
	},
	{
		id: 'logout',
		label: 'Cerrar Sesión',
		description: 'Salir de la cuenta Megabrisas',
		icon: LogOut,
		category: 'action',
		subCategory: 'settings',
		keywords: ['salir', 'cerrar', 'sesión', 'logout', 'desconectar'],
		shortcut: 'Ctrl+Q'
	},
	{
		id: 'show-shortcuts',
		label: 'Atajos de Teclado',
		description: 'Ver lista de comandos disponibles',
		icon: Keyboard,
		category: 'action',
		subCategory: 'settings',
		keywords: ['atajos', 'teclado', 'shortcuts', 'comandos', 'ayuda', 'teclas'],
		shortcut: 'Shift+A'
	}
];

// ============================================
// MODULE COMPONENT KEY MAPPING
// ============================================

export const MODULE_COMPONENT_MAP: Record<string, { componentKey: string; title: string }> = {
	'users-list': { componentKey: 'user-list', title: 'Lista Usuarios' },
	'ingreso-list': { componentKey: 'ingreso-list', title: 'Ingresos Contratistas' },
	'contratista-list': { componentKey: 'contratista-list', title: 'Lista Contratistas' },
	'proveedor-ingreso-list': {
		componentKey: 'proveedor-ingreso-list',
		title: 'Ingresos Proveedores'
	},
	'proveedor-list': { componentKey: 'proveedor-list', title: 'Lista Proveedores' },
	'visitas-list': { componentKey: 'visitas-list', title: 'Ingresos Visitas' },
	'visitante-list': { componentKey: 'visitante-list', title: 'Lista Visitantes' },
	'gafete-list': { componentKey: 'gafete-list', title: 'Lista Gafetes' },
	'alerta-gafete-list': { componentKey: 'alerta-gafete-list', title: 'Control de Alertas' },
	'lista-negra-list': { componentKey: 'lista-negra-list', title: 'Lista Negra' },
	// Settings mappings
	'settings-general': { componentKey: 'general-settings', title: 'Ajustes Generales' },
	'settings-visual': { componentKey: 'visual-settings', title: 'Ajustes Gráficos' },
	'settings-session': { componentKey: 'session-settings', title: 'Gestión de Sesión' },
	'settings-roles': { componentKey: 'roles-settings', title: 'Roles y Permisos' },
	'settings-shortcuts': { componentKey: 'shortcuts-settings', title: 'Atajos de Teclado' },
	'settings-export': { componentKey: 'export-settings', title: 'Configuración de Exportación' },
	'settings-backup': { componentKey: 'backup-settings', title: 'Copias de Seguridad' },
	'settings-spotlight': { componentKey: 'spotlight-settings', title: 'Ajustes de Spotlight' },
	welcome: { componentKey: 'welcome', title: 'Bienvenida' }
};

// ============================================
// ICON FOR TABS
// ============================================

export const TAB_ICON: any = ArrowRight;
