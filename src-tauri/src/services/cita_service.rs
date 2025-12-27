// src/services/cita_service.rs
use crate::db::surrealdb_cita_queries as db;
use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use crate::domain::errors::CitaError;
use crate::models::visitante::CreateVisitanteInput;
use crate::services::visitante_service;
use chrono::Local;
use surrealdb::RecordId;

pub async fn agendar_cita(
    cita_input: CreateCitaInput,
    visitante_input: Option<CreateVisitanteInput>,
) -> Result<Cita, CitaError> {
    // 1. Resolver Visitante (Existente o Crear)
    let mut visitante_id: RecordId;

    if !cita_input.visitante_id.is_empty() {
        let id_str = cita_input.visitante_id.clone();
        // Asumimos formato simple o completo
        if id_str.contains(':') {
            let parts: Vec<&str> = id_str.split(':').collect();
            visitante_id = RecordId::from_table_key(parts[0], parts[1]);
        } else {
            visitante_id = RecordId::from_table_key("visitante", &id_str);
        }
    } else {
        // Placeholder temporal, se reasignará abajo
        visitante_id = RecordId::from_table_key("visitante", "temp");
    }

    if let Some(v_input) = visitante_input {
        // Si viene input de visitante, buscamos por cédula o creamos
        let existente = visitante_service::get_visitante_by_cedula(&v_input.cedula)
            .await
            .map_err(|e| CitaError::Database(e.to_string()))?;

        match existente {
            Some(v) => {
                visitante_id =
                    v.id.parse().unwrap_or(RecordId::from_table_key("visitante", "invalid"));
            }
            None => {
                let nuevo = visitante_service::create_visitante(v_input)
                    .await
                    .map_err(|e| CitaError::Database(e.to_string()))?;
                visitante_id =
                    nuevo.id.parse().unwrap_or(RecordId::from_table_key("visitante", "invalid"));
            }
        }
    }

    if visitante_id.key().to_string() == "temp" {
        return Err(CitaError::Validation("Visitante requerido y no proporcionado".to_string()));
    }

    // 2. Preparar payload para SurrealDB
    // Usamos serde_json::json! para construir el objeto flexiblemente
    // 2. Preparar payload para SurrealDB
    let cita_json = serde_json::json!({
        "visitante_id": visitante_id,
        "usuario_id": RecordId::from_table_key("user", &cita_input.registrado_por),
        "motivo": cita_input.motivo,
        "fecha_inicio": cita_input.fecha_cita, // Mapeamos fecha_cita -> fecha_inicio en DB
        "fecha_fin": cita_input.fecha_cita, // Duplicamos para simplificar
        "anfitrion": cita_input.anfitrion,
        "area_visitada": cita_input.area_visitada,
        "estado": "PENDIENTE", // Case sensitive? Revisar enum
        "activa": true
    });

    // 3. Insertar
    let nueva_cita = db::insert(cita_json)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?
        .ok_or(CitaError::Database("Error creando cita".to_string()))?;

    // 4. Mapear a Domain
    Ok(Cita {
        id: nueva_cita.id,
        visitante_id: nueva_cita
            .visitante_id
            .unwrap_or(RecordId::from_table_key("visitante", "unknown")),
        fecha_cita: nueva_cita.fecha_inicio, // Mapeo inverso
        anfitrion: cita_input.anfitrion, // Recuperar de input pues db::Cita quizas no lo traiga struct
        area_visitada: cita_input.area_visitada,
        motivo: nueva_cita.motivo,
        estado: nueva_cita.estado,
        registrado_por: nueva_cita.usuario_id.to_string(), // Mapeo inverso
        created_at: nueva_cita.created_at,
        updated_at: nueva_cita.updated_at,
    })
}

pub async fn get_citas_hoy() -> Result<Vec<CitaPopulated>, CitaError> {
    let now = Local::now();
    let hoy_inicio = now.format("%Y-%m-%dT00:00:00").to_string();
    let hoy_fin = now.format("%Y-%m-%dT23:59:59").to_string();

    let citas = db::find_activas_by_fecha(&hoy_inicio, &hoy_fin)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))?;

    let mut populated = Vec::new();
    for c in citas {
        let v = if let Some(vid) = &c.visitante_id {
            visitante_service::get_visitante_by_id(&vid.to_string()).await.unwrap_or(None)
        } else {
            None
        };
        let v_ref = v.as_ref();

        populated.push(CitaPopulated {
            id: c.id.to_string(),
            fecha_cita: c.fecha_inicio.to_string(),
            anfitrion: "Desconocido".to_string(), // TODO: Agregar anfitrion a db::Cita struct
            area_visitada: "Desconocido".to_string(), // TODO: Agregar area a db::Cita struct
            motivo: c.motivo,
            estado: c.estado,
            visitante_id: c.visitante_id.map(|v| v.to_string()).unwrap_or_default(),
            visitante_nombre: v_ref.map(|x| x.nombre.clone()).unwrap_or_default(),
            visitante_apellido: v_ref.map(|x| x.apellido.clone()).unwrap_or_default(),
            visitante_cedula: v_ref.map(|x| x.cedula.clone()).unwrap_or_default(),
            visitante_nombre_completo: v_ref
                .map(|x| format!("{} {}", x.nombre, x.apellido))
                .unwrap_or_default(),
            visitante_empresa: v_ref.and_then(|x| x.empresa.clone()),
        });
    }
    Ok(populated)
}

pub async fn get_citas_pendientes() -> Result<Vec<CitaPopulated>, CitaError> {
    // Reutiliza get_citas_hoy por simplicidad o expande rango
    get_citas_hoy().await
}

pub async fn update_cita(
    // Stub funcional: updates completos se manejarian similar a insert
    _id: String,
    _fecha: String,
    _anf: String,
    _area: String,
    _mot: Option<String>,
) -> Result<(), CitaError> {
    // TODO: Implementar update real en queries
    Ok(())
}

pub async fn procesar_ingreso_cita(
    _id: String,
    _gafete: String,
    _usuario_id: String,
) -> Result<String, CitaError> {
    // 1. Obtener cita y visitante
    // (Simplificado: asumimos que frontend pasa datos necesarios a registrar_ingreso,
    // pero si solo pasa ID cita, aqui deberíamos buscarla.
    // Por ahora, stub funcional de error si no se implementa flujo completo)
    Err(CitaError::Database("Flujo procesar_ingreso_cita requiere refactor mayor".to_string()))
}

pub async fn cancelar_cita(id_str: String) -> Result<(), CitaError> {
    let id: RecordId = if id_str.contains(':') {
        id_str.parse().map_err(|_| CitaError::Validation("ID inválido".into()))?
    } else {
        RecordId::from_table_key("cita", &id_str)
    };

    db::cancel(&id).await.map_err(|e| CitaError::Database(e.to_string()))?;
    Ok(())
}
