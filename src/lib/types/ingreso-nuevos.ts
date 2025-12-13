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
    placa_vehiculo?: string;
    observaciones?: string;
    usuario_ingreso_id: string;
}
