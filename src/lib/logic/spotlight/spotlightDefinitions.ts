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
    ArrowRight
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
    },
    {
        id: 'trash-list',
        label: 'Papelera',
        description: 'Elementos eliminados',
        icon: Trash2,
        category: 'module',
        keywords: ['papelera', 'trash', 'eliminado', 'borrado'],
        permission: 'VIEW_TRASH'
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
        id: 'open-settings',
        label: 'Configuración',
        description: 'Abrir panel de configuración',
        icon: Settings,
        category: 'action',
        keywords: ['configuracion', 'settings', 'ajustes', 'opciones']
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
    'trash-list': { componentKey: 'trash-list', title: 'Papelera' }
};

// ============================================
// ICON FOR TABS
// ============================================

export const TAB_ICON: any = ArrowRight;
