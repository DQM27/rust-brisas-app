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
    Ban
} from 'lucide-svelte';
import type { SpotlightItemDefinition } from '$lib/types/spotlight';

// ============================================
// MODULE DEFINITIONS
// ============================================

export const MODULE_DEFINITIONS: SpotlightItemDefinition[] = [
    {
        id: 'users-list',
        label: 'Usuarios',
        description: 'Gestión de usuarios del sistema',
        icon: UserCircle2,
        category: 'module',
        keywords: ['user', 'usuarios', 'lista', 'administrar'],
        permission: 'VIEW_USER_DETAIL'
    },
    {
        id: 'ingreso-list',
        label: 'Ingresos Contratista',
        description: 'Control de ingresos de contratistas',
        icon: DoorOpen,
        category: 'module',
        keywords: ['ingreso', 'contratista', 'entrada', 'acceso'],
        permission: 'VIEW_ENTRY_LIST'
    },
    {
        id: 'proveedor-ingreso-list',
        label: 'Ingresos Proveedor',
        description: 'Control de ingresos de proveedores',
        icon: PackageCheck,
        category: 'module',
        keywords: ['proveedor', 'ingreso', 'suministro', 'entrada'],
        permission: 'VIEW_ENTRY_LIST'
    },
    {
        id: 'visitas-list',
        label: 'Visitas',
        description: 'Gestión de visitantes',
        icon: Contact,
        category: 'module',
        keywords: ['visita', 'visitante', 'invitado', 'entrada'],
        permission: 'VIEW_VISITOR_LIST'
    },
    {
        id: 'gafete-list',
        label: 'Gafetes',
        description: 'Gestión de gafetes e identificaciones',
        icon: IdCard,
        category: 'module',
        keywords: ['gafete', 'identificacion', 'tarjeta', 'badge'],
        permission: 'VIEW_GAFETE_LIST'
    },
    {
        id: 'lista-negra-list',
        label: 'Lista Negra',
        description: 'Control de acceso restringido',
        icon: ShieldX,
        category: 'module',
        keywords: ['lista negra', 'blacklist', 'bloqueado', 'restringido'],
        permission: 'VIEW_BLACKLIST'
    }
];

// ============================================
// ACTION DEFINITIONS
// ============================================

export const ACTION_DEFINITIONS: SpotlightItemDefinition[] = [
    {
        id: 'create-contratista',
        label: 'Nuevo Ingreso Contratista',
        description: 'Registrar nuevo ingreso de contratista',
        icon: Plus,
        category: 'action',
        keywords: ['nuevo', 'crear', 'contratista', 'ingreso', 'registrar'],
        permission: 'CREATE_ENTRY'
    },
    {
        id: 'create-user',
        label: 'Crear Nuevo Usuario',
        description: 'Registrar un nuevo usuario en el sistema',
        icon: UserPlus,
        category: 'action',
        keywords: ['nuevo', 'crear', 'usuario', 'user', 'registrar', 'empleado'],
        permission: 'EDIT_USER'
    },
    {
        id: 'create-proveedor',
        label: 'Nuevo Ingreso Proveedor',
        description: 'Registrar nuevo ingreso de proveedor',
        icon: Plus,
        category: 'action',
        keywords: ['nuevo', 'crear', 'proveedor', 'ingreso', 'registrar'],
        permission: 'CREATE_ENTRY'
    },
    {
        id: 'create-visita',
        label: 'Nueva Visita',
        description: 'Registrar nueva visita',
        icon: Plus,
        category: 'action',
        keywords: ['nuevo', 'crear', 'visita', 'visitante', 'registrar'],
        permission: 'CREATE_VISITOR'
    },
    {
        id: 'create-blacklist',
        label: 'Bloquear Persona',
        description: 'Añadir persona a lista negra',
        icon: Ban,
        category: 'action',
        keywords: ['bloquear', 'banear', 'lista negra', 'denegar', 'restringir'],
        permission: 'MANAGE_BLACKLIST'
    },
    {
        id: 'create-gafete',
        label: 'Nuevo Gafete',
        description: 'Registrar un nuevo gafete',
        icon: Plus,
        category: 'action',
        keywords: ['nuevo', 'crear', 'gafete', 'carnet', 'tarjeta', 'inventario'],
        permission: 'VIEW_GAFETE_LIST'
    },
    // Configuración
    {
        id: 'settings-general',
        label: 'Ajustes Generales',
        description: 'Preferencias globales del sistema',
        icon: Settings,
        category: 'action',
        keywords: ['configuracion', 'ajustes', 'general', 'sistema'],
        permission: 'VIEW_SETTINGS_GENERAL'
    },
    {
        id: 'settings-visual',
        label: 'Ajustes Gráficos',
        description: 'Configuración visual y temas',
        icon: Monitor,
        category: 'action',
        keywords: ['configuracion', 'ajustes', 'graficos', 'visual', 'tema', 'modo oscuro'],
        permission: 'VIEW_SETTINGS_VISUAL'
    },
    {
        id: 'settings-session',
        label: 'Gestión de Sesión',
        description: 'Configuración de seguridad y sesiones',
        icon: ShieldCheck,
        category: 'action',
        keywords: ['configuracion', 'ajustes', 'sesion', 'seguridad', 'password'],
        permission: 'VIEW_SETTINGS_SESSIONS'
    },
    {
        id: 'settings-roles',
        label: 'Roles y Permisos',
        description: 'Gestión de roles de usuario',
        icon: Users,
        category: 'action',
        keywords: ['configuracion', 'roles', 'permisos', 'usuarios', 'acceso'],
        permission: 'VIEW_ROLE_LIST'
    },
    {
        id: 'settings-export',
        label: 'Configuración de Exportación',
        description: 'Ajustes de reportes y exportación',
        icon: Download,
        category: 'action',
        keywords: ['configuracion', 'exportacion', 'reportes', 'excel', 'pdf'],
        permission: 'VIEW_SETTINGS_BACKUP'
    },
    {
        id: 'settings-backup',
        label: 'Copias de Seguridad',
        description: 'Gestión de respaldos del sistema',
        icon: Database,
        category: 'action',
        keywords: ['configuracion', 'backup', 'respaldo', 'copia', 'seguridad'],
        permission: 'VIEW_SETTINGS_BACKUP'
    },
    {
        id: 'trash-settings',
        label: 'Papelera',
        description: 'Gestión de elementos eliminados',
        icon: Trash2,
        category: 'action',
        keywords: ['papelera', 'trash', 'eliminado', 'restaurar'],
        permission: 'VIEW_TRASH'
    },
    {
        id: 'action-reindex',
        label: 'Reindexar Búsqueda',
        description: 'Regenerar índices de búsqueda',
        icon: RefreshCw,
        category: 'action',
        keywords: ['reindexar', 'busqueda', 'indice', 'regenerar'],
        // Permiso de superusuario o admin (se validará en lógica)
    }
];

// ============================================
// MODULE COMPONENT KEY MAPPING
// ============================================

export const MODULE_COMPONENT_MAP: Record<string, { componentKey: string; title: string }> = {
    'users-list': { componentKey: 'user-list', title: 'Lista de Usuarios' },
    'ingreso-list': { componentKey: 'ingreso-list', title: 'Ingresos Contratista' },
    'proveedor-ingreso-list': { componentKey: 'proveedor-ingreso-list', title: 'Ingresos Proveedor' },
    'visitas-list': { componentKey: 'visitas-list', title: 'Ingreso Visitas' },
    'gafete-list': { componentKey: 'gafete-list', title: 'Gestión de Gafetes' },
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
