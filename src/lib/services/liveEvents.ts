// ==========================================
// src/lib/services/liveEvents.ts
// Frontend Live Events Handler for SurrealDB LIVE Queries
// ==========================================
//
// Este servicio escucha eventos Tauri emitidos por el backend
// cuando los datos cambian en tiempo real en SurrealDB.
//
// Uso:
//   import { onIngresoChange, LIVE_EVENTS } from '$lib/services/liveEvents';
//   
//   onMount(async () => {
//     const unlisten = await onIngresoChange((notification) => {
//       console.log('Ingreso cambió:', notification);
//       // Recargar datos o actualizar UI
//     });
//     return unlisten;
//   });
//

import { listen, type UnlistenFn } from '@tauri-apps/api/event';

// ==========================================
// TIPOS
// ==========================================

/** Acciones posibles en una notificación LIVE */
export type LiveAction = 'create' | 'update' | 'delete';

/** Payload de una notificación LIVE */
export interface LiveNotification<T = unknown> {
    action: LiveAction;
    table: string;
    data: T;
}

/** Callback para manejar notificaciones */
export type LiveNotificationHandler<T = unknown> = (notification: LiveNotification<T>) => void;

// ==========================================
// NOMBRES DE EVENTOS
// ==========================================

/** Nombres de eventos que coinciden con los definidos en el backend */
export const LIVE_EVENTS = {
    /** Cambios en la tabla ingreso */
    INGRESO: 'ingreso:changed',
    /** Cambios en alertas de gafetes */
    ALERTA_GAFETE: 'alerta_gafete:changed',
    /** Cambios en disponibilidad de gafetes */
    GAFETE: 'gafete:changed',
    /** Cambios en contratistas */
    CONTRATISTA: 'contratista:changed',
    /** Cambios en proveedores */
    PROVEEDOR: 'proveedor:changed',
} as const;

export type LiveEventName = typeof LIVE_EVENTS[keyof typeof LIVE_EVENTS];

// ==========================================
// FUNCIONES HELPER
// ==========================================

/**
 * Suscribirse a cambios de una tabla específica.
 * 
 * @param eventName - Nombre del evento (usar LIVE_EVENTS.*)
 * @param handler - Callback que recibe la notificación
 * @returns Función para cancelar la suscripción
 * 
 * @example
 * ```ts
 * const unlisten = await onTableChange(LIVE_EVENTS.INGRESO, (notification) => {
 *     if (notification.action === 'create') {
 *         console.log('Nuevo ingreso:', notification.data);
 *     }
 * });
 * // Cuando el componente se desmonte:
 * unlisten();
 * ```
 */
export async function onTableChange<T = unknown>(
    eventName: LiveEventName,
    handler: LiveNotificationHandler<T>
): Promise<UnlistenFn> {
    return await listen<LiveNotification<T>>(eventName, (event) => {
        handler(event.payload);
    });
}

// ==========================================
// HELPERS ESPECÍFICOS POR TABLA
// ==========================================

/**
 * Suscribirse a cambios en la tabla ingreso.
 * Útil para dashboards en tiempo real.
 */
export async function onIngresoChange<T = unknown>(
    handler: LiveNotificationHandler<T>
): Promise<UnlistenFn> {
    return onTableChange(LIVE_EVENTS.INGRESO, handler);
}

/**
 * Suscribirse a cambios en alertas de gafetes.
 * Útil para notificaciones en tiempo real.
 */
export async function onAlertaGafeteChange<T = unknown>(
    handler: LiveNotificationHandler<T>
): Promise<UnlistenFn> {
    return onTableChange(LIVE_EVENTS.ALERTA_GAFETE, handler);
}

/**
 * Suscribirse a cambios en gafetes.
 * Útil para tracking de disponibilidad.
 */
export async function onGafeteChange<T = unknown>(
    handler: LiveNotificationHandler<T>
): Promise<UnlistenFn> {
    return onTableChange(LIVE_EVENTS.GAFETE, handler);
}

/**
 * Suscribirse a cambios en contratistas.
 */
export async function onContratistaChange<T = unknown>(
    handler: LiveNotificationHandler<T>
): Promise<UnlistenFn> {
    return onTableChange(LIVE_EVENTS.CONTRATISTA, handler);
}

/**
 * Suscribirse a cambios en proveedores.
 */
export async function onProveedorChange<T = unknown>(
    handler: LiveNotificationHandler<T>
): Promise<UnlistenFn> {
    return onTableChange(LIVE_EVENTS.PROVEEDOR, handler);
}

// ==========================================
// UTILIDAD PARA DEBUG
// ==========================================

/**
 * Suscribirse a TODOS los eventos LIVE para debugging.
 * No usar en producción.
 * 
 * @returns Array de funciones para cancelar todas las suscripciones
 */
export async function debugListenAll(): Promise<UnlistenFn[]> {
    const eventNames = Object.values(LIVE_EVENTS);

    const unlistenFunctions = await Promise.all(
        eventNames.map(eventName =>
            listen<LiveNotification>(eventName, (event) => {
                console.log(`[LIVE DEBUG] ${eventName}:`, event.payload);
            })
        )
    );

    console.log(`[LIVE DEBUG] Escuchando ${eventNames.length} eventos LIVE`);

    return unlistenFunctions;
}
