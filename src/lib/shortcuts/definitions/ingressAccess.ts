/**
 * Atajos de Acceso a Módulos de Ingreso
 */
import type { ShortcutDefinition } from '../types';
import { openView } from '$lib/stores/sidebar';

export const ingressAccessShortcuts: ShortcutDefinition[] = [
    {
        id: 'open-ingresos-contratista',
        keys: 'alt+1',
        label: 'Ver Ingresos Contratista',
        description: 'Abre el listado de ingresos de contratistas',
        category: 'ingress-access',
        scope: 'all',
        icon: 'DoorOpen',
        handler: (e) => {
            e.preventDefault();
            openView('ingreso-list', 'Ingresos Contratista');
        }
    },
    {
        id: 'open-ingresos-proveedor',
        keys: 'alt+2',
        label: 'Ver Ingresos Proveedor',
        description: 'Abre el listado de ingresos de proveedores',
        category: 'ingress-access',
        scope: 'all',
        icon: 'PackageCheck',
        handler: (e) => {
            e.preventDefault();
            openView('proveedor-ingreso-list', 'Ingresos Proveedor');
        }
    },
    {
        id: 'open-ingresos-visita',
        keys: 'alt+3',
        label: 'Ver Ingreso Visitas',
        description: 'Abre el listado de ingresos de visitas',
        category: 'ingress-access',
        scope: 'all',
        icon: 'Contact',
        handler: (e) => {
            e.preventDefault();
            openView('visitas-list', 'Ingreso Visitas');
        }
    },
    {
        id: 'open-alertas',
        keys: 'alt+a',
        label: 'Ver Alertas',
        description: 'Abre el panel de control de alertas',
        category: 'ingress-access',
        scope: 'all',
        icon: 'AlertTriangle',
        handler: (e) => {
            e.preventDefault();
            openView('alerta-gafete-list', 'Control de Alertas');
        }
    }
];
