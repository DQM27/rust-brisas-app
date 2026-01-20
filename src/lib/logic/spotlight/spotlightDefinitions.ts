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
    Trash2,
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
    ListPlus
} from 'lucide-svelte';
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
        permission: 'VIEW_USER_DETAIL'
    },
    {
        id: 'ingreso-list',
        label: 'Ingresos Contratistas',
        description: 'Control de ingresos de contratistas',
        icon: DoorOpen,
        category: 'module',
        subCategory: 'link',
        keywords: ['ingreso', 'contratista', 'entrada', 'acceso'],
        permission: 'VIEW_ENTRY_LIST'
    },
    {
        id: 'contratista-list',
        label: 'Lista Contratistas',
        description: 'Gestión de catálogo de contratistas',
        icon: Users,
        category: 'module',
        subCategory: 'link',
        keywords: ['lista', 'contratista', 'maestro', 'administrar'],
        permission: 'VIEW_ENTRY_LIST'
    },
    {
        id: 'proveedor-ingreso-list',
        label: 'Ingresos Proveedores',
        description: 'Control de ingresos de proveedores',
        icon: PackageCheck,
        category: 'module',
        subCategory: 'link',
        keywords: ['proveedor', 'ingreso', 'suministro', 'entrada'],
        permission: 'VIEW_ENTRY_LIST'
    },
    {
        id: 'proveedor-list',
        label: 'Lista Proveedores',
        description: 'Gestión de catálogo de proveedores',
        icon: PackageCheck,
        category: 'module',
        subCategory: 'link',
        keywords: ['lista', 'proveedor', 'maestro', 'administrar'],
        permission: 'VIEW_ENTRY_LIST'
    },
    {
        id: 'visitas-list',
        label: 'Ingresos Visitas',
        description: 'Control de ingresos de visitantes',
        icon: Contact,
        category: 'module',
        subCategory: 'link',
        keywords: ['visita', 'visitante', 'invitado', 'entrada'],
        permission: 'VIEW_VISITOR_LIST'
    },
    {
        id: 'visitante-list',
        label: 'Lista Visitantes',
        description: 'Catálogo histórico de visitantes',
        icon: Users,
        category: 'module',
        subCategory: 'link',
        keywords: ['lista', 'visitante', 'maestro', 'administrar'],
        permission: 'VIEW_VISITOR_LIST'
    },
    {
        id: 'gafete-list',
        label: 'Lista Gafetes',
        description: 'Gestión de inventario de gafetes',
        icon: IdCard,
        category: 'module',
        subCategory: 'link',
        keywords: ['gafete', 'identificacion', 'tarjeta', 'badge'],
        permission: 'VIEW_GAFETE_LIST'
    },
    {
        id: 'lista-negra-list',
        label: 'Lista Negra',
        description: 'Control de acceso restringido',
        icon: ShieldX,
        category: 'module',
        subCategory: 'link',
        keywords: ['lista negra', 'blacklist', 'bloqueado', 'restringido'],
        permission: 'VIEW_BLACKLIST'
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
        permission: 'CREATE_ENTRY'
    },
    {
        id: 'create-proveedor',
        label: 'Nuevo Ingreso Proveedor',
        description: 'Registrar entrada de proveedor',
        icon: Plus,
        category: 'action',
        subCategory: 'transaction',
        keywords: ['nuevo', 'crear', 'proveedor', 'ingreso', 'registrar'],
        permission: 'CREATE_ENTRY'
    },
    {
        id: 'create-visita',
        label: 'Nuevo Ingreso Visita',
        description: 'Registrar entrada de visitante',
        icon: Plus,
        category: 'action',
        subCategory: 'transaction',
        keywords: ['nuevo', 'crear', 'visita', 'visitante', 'ingreso', 'registrar'],
        permission: 'CREATE_VISITOR'
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
        permission: 'CREATE_ENTRY'
    },
    {
        id: 'master-proveedor',
        label: 'Crear Proveedor',
        description: 'Añadir nuevo proveedor al catálogo',
        icon: UserPlus,
        category: 'action',
        subCategory: 'master',
        keywords: ['crear', 'maestro', 'catálogo', 'proveedor'],
        permission: 'CREATE_ENTRY'
    },
    {
        id: 'master-visitante',
        label: 'Crear Visita',
        description: 'Añadir nuevo visitante al catálogo',
        icon: UserPlus,
        category: 'action',
        subCategory: 'master',
        keywords: ['crear', 'maestro', 'catálogo', 'visitante', 'visita'],
        permission: 'CREATE_VISITOR'
    },
    {
        id: 'create-user',
        label: 'Crear Usuario',
        description: 'Registrar un nuevo acceso al sistema',
        icon: UserPlus,
        category: 'action',
        subCategory: 'master',
        keywords: ['nuevo', 'crear', 'usuario', 'user', 'registrar', 'empleado'],
        permission: 'CREATE_USER'
    },
    {
        id: 'create-blacklist',
        label: 'Crear Bloqueo',
        description: 'Añadir persona a lista negra',
        icon: Ban,
        category: 'action',
        subCategory: 'master',
        keywords: ['bloquear', 'banear', 'lista negra', 'denegar', 'restringir'],
        permission: 'MANAGE_BLACKLIST'
    },
    {
        id: 'create-gafete',
        label: 'Crear Gafete',
        description: 'Registrar un nuevo gafete físico',
        icon: Plus,
        category: 'action',
        subCategory: 'master',
        keywords: ['nuevo', 'crear', 'gafete', 'carnet', 'tarjeta', 'inventario'],
        permission: 'CREATE_GAFETE'
    },
    {
        id: 'create-gafete-batch',
        label: 'Crear Lote de Gafetes',
        description: 'Generar múltiples gafetes por rango',
        icon: ListPlus,
        category: 'action',
        subCategory: 'master',
        keywords: ['lote', 'rango', 'generar', 'masivo', 'gafetes'],
        permission: 'CREATE_GAFETE'
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
        permission: 'VIEW_SETTINGS_GENERAL'
    },
    {
        id: 'settings-visual',
        label: 'Ajustes Gráficos',
        description: 'Configuración visual y temas',
        icon: Monitor,
        category: 'action',
        subCategory: 'settings',
        keywords: ['configuracion', 'ajustes', 'graficos', 'visual', 'tema', 'modo oscuro'],
        permission: 'VIEW_SETTINGS_VISUAL'
    },
    {
        id: 'settings-session',
        label: 'Gestión de Sesión',
        description: 'Configuración de seguridad y sesiones',
        icon: ShieldCheck,
        category: 'action',
        subCategory: 'settings',
        keywords: ['configuracion', 'ajustes', 'sesion', 'seguridad', 'password'],
        permission: 'VIEW_SETTINGS_SESSIONS'
    },
    {
        id: 'settings-roles',
        label: 'Roles y Permisos',
        description: 'Gestión de roles de usuario',
        icon: Users,
        category: 'action',
        subCategory: 'settings',
        keywords: ['configuracion', 'roles', 'permisos', 'usuarios', 'acceso'],
        permission: 'VIEW_ROLE_LIST'
    },
    {
        id: 'settings-export',
        label: 'Configuración de Exportación',
        description: 'Ajustes de reportes y exportación',
        icon: Download,
        category: 'action',
        subCategory: 'settings',
        keywords: ['configuracion', 'exportacion', 'reportes', 'excel', 'pdf'],
        permission: 'VIEW_SETTINGS_BACKUP'
    },
    {
        id: 'settings-backup',
        label: 'Copias de Seguridad',
        description: 'Gestión de respaldos del sistema',
        icon: Database,
        category: 'action',
        subCategory: 'settings',
        keywords: ['configuracion', 'backup', 'respaldo', 'copia', 'seguridad'],
        permission: 'VIEW_SETTINGS_BACKUP'
    },
    {
        id: 'trash-settings',
        label: 'Papelera',
        description: 'Gestión de elementos eliminados',
        icon: Trash2,
        category: 'action',
        subCategory: 'link',
        keywords: ['papelera', 'trash', 'eliminado', 'restaurar'],
        permission: 'VIEW_TRASH'
    },
    {
        id: 'action-reindex',
        label: 'Reindexar Búsqueda',
        description: 'Regenerar índices de búsqueda',
        icon: RefreshCw,
        category: 'action',
        subCategory: 'link',
        keywords: ['reindexar', 'busqueda', 'indice', 'regenerar'],
        // Permiso de superusuario o admin (se validará en lógica)
    }
];

// ============================================
// MODULE COMPONENT KEY MAPPING
// ============================================

export const MODULE_COMPONENT_MAP: Record<string, { componentKey: string; title: string }> = {
    'users-list': { componentKey: 'user-list', title: 'Lista Usuarios' },
    'ingreso-list': { componentKey: 'ingreso-list', title: 'Ingresos Contratistas' },
    'contratista-list': { componentKey: 'contratista-list', title: 'Lista Contratistas' },
    'proveedor-ingreso-list': { componentKey: 'proveedor-ingreso-list', title: 'Ingresos Proveedores' },
    'proveedor-list': { componentKey: 'proveedor-list', title: 'Lista Proveedores' },
    'visitas-list': { componentKey: 'visitas-list', title: 'Ingresos Visitas' },
    'visitante-list': { componentKey: 'visitante-list', title: 'Lista Visitantes' },
    'gafete-list': { componentKey: 'gafete-list', title: 'Lista Gafetes' },
    'lista-negra-list': { componentKey: 'lista-negra-list', title: 'Lista Negra' },
    // Settings mappings
    'settings-general': { componentKey: 'general-settings', title: 'Ajustes Generales' },
    'settings-visual': { componentKey: 'visual-settings', title: 'Ajustes Gráficos' },
    'settings-session': { componentKey: 'session-settings', title: 'Gestión de Sesión' },
    'settings-roles': { componentKey: 'roles-settings', title: 'Roles y Permisos' },
    'settings-export': { componentKey: 'export-settings', title: 'Configuración de Exportación' },
    'settings-backup': { componentKey: 'backup-settings', title: 'Copias de Seguridad' },
    'trash-settings': { componentKey: 'trash-settings', title: 'Papelera de Reciclaje' },
};

// ============================================
// ICON FOR TABS
// ============================================

export const TAB_ICON: any = ArrowRight;
