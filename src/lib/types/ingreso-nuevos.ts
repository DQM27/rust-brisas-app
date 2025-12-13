export interface IngresoVisita {
    id: string;
    visitanteId: string;
    citaId?: string;
    anfitrion: string;
    areaVisitada: string;
    motivo: string;
    gafete?: string;
    fechaIngreso: string;
    fechaSalida?: string;
    estado: 'ADENTRO' | 'SALIO';
    usuarioIngresoId: string;
    usuarioSalidaId?: string;
    observaciones?: string;

    // Populated fields
    visitanteNombre: string;
    visitanteApellido: string;
    visitanteCedula: string;
    visitanteEmpresa?: string;
}

export interface IngresoProveedor {
    id: string;
    cedula: string;
    nombre: string;
    apellido: string;
    empresaId: string;
    areaVisitada: string;
    motivo: string;
    gafete?: string;
    tipoAutorizacion?: string;
    modoIngreso?: string;
    placaVehiculo?: string;
    fechaIngreso: string;
    fechaSalida?: string;
    estado: 'ADENTRO' | 'SALIO';
    usuarioIngresoId: string;
    usuarioSalidaId?: string;
    observaciones?: string;
}

export interface CreateIngresoProveedorInput {
    cedula: string;
    nombre: string;
    apellido: string;
    empresa_id: string;
    area_visitada: string;
    motivo: string;
    gafete?: string;
    tipo_autorizacion: string;
    modo_ingreso: string;
    // Vehicle fields
    tipo_vehiculo?: string;
    placa_vehiculo?: string;
    marca_vehiculo?: string;
    modelo_vehiculo?: string;
    color_vehiculo?: string;
    // Audit
    observaciones?: string;
    usuario_ingreso_id: string;
}

export interface ProveedorCatalogItem {
    id: string;
    cedula: string;
    nombre: string;
    apellido: string;
    empresaId: string;
    empresaNombre: string;
    // Vehicle info
    vehiculoTipo?: string;
    vehiculoPlaca?: string;
    vehiculoMarca?: string;
    vehiculoModelo?: string;
    vehiculoColor?: string;
}
