// ==========================================
// src/services/email_service.rs
// ==========================================
// Capa de servicio: orquesta dominio y db
// Contiene la logica de negocio para envio de emails y reportes

use crate::db::reporte_queries as db;
use crate::models::reporte::{
    CreateReporteInput, EstadoReporte, ReporteListResponse, ReporteResponse, TipoReporte,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Utc;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use sqlx::SqlitePool;
use std::env;
use uuid::Uuid;

// ==========================================
// CREAR Y ENVIAR REPORTE
// ==========================================

pub async fn crear_y_enviar_reporte(
    pool: &SqlitePool,
    input: CreateReporteInput,
) -> Result<ReporteResponse, String> {
    // 1. Validar tipo
    let tipo = TipoReporte::from_str(&input.tipo)
        .ok_or_else(|| "Tipo de reporte invalido. Use: error, sugerencia, mejora".to_string())?;

    // 2. Validar campos obligatorios
    if input.asunto.trim().is_empty() {
        return Err("El asunto es obligatorio".to_string());
    }
    if input.mensaje.trim().is_empty() {
        return Err("El mensaje es obligatorio".to_string());
    }

    // 3. Generar ID y timestamps
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let tiene_adjunto = input.adjunto_base64.is_some() && input.nombre_adjunto.is_some();

    // 4. Guardar en DB con estado pendiente
    db::insert(
        pool,
        &id,
        tipo.as_str(),
        input.asunto.trim(),
        input.mensaje.trim(),
        input.contacto.as_deref(),
        tiene_adjunto,
        input.nombre_adjunto.as_deref(),
        EstadoReporte::Pendiente.as_str(),
        &now,
        &now,
    )
    .await?;

    // 5. Intentar enviar email
    let resultado_envio = enviar_email(
        &input.asunto,
        &input.mensaje,
        input.contacto.as_deref(),
        input.adjunto_base64.as_deref(),
        input.nombre_adjunto.as_deref(),
    )
    .await;

    // 6. Actualizar estado segun resultado
    let now_updated = Utc::now().to_rfc3339();
    match resultado_envio {
        Ok(_) => {
            db::update_estado(
                pool,
                &id,
                EstadoReporte::Enviado.as_str(),
                None,
                Some(&now_updated),
                &now_updated,
            )
            .await?;
        }
        Err(ref e) => {
            db::update_estado(
                pool,
                &id,
                EstadoReporte::Fallido.as_str(),
                Some(e),
                None,
                &now_updated,
            )
            .await?;
        }
    }

    // 7. Obtener y retornar reporte actualizado
    let reporte = db::find_by_id(pool, &id).await?;

    // Si el envio fallo, retornar error pero el reporte quedo guardado
    if let Err(e) = resultado_envio {
        return Err(format!(
            "Reporte guardado pero fallo el envio: {}. ID: {}",
            e, id
        ));
    }

    Ok(ReporteResponse::from(reporte))
}

// ==========================================
// OBTENER TODOS LOS REPORTES
// ==========================================

pub async fn get_all_reportes(pool: &SqlitePool) -> Result<ReporteListResponse, String> {
    let reportes = db::find_all(pool).await?;

    let total = reportes.len();
    let enviados = reportes
        .iter()
        .filter(|r| r.estado == EstadoReporte::Enviado)
        .count();
    let pendientes = reportes
        .iter()
        .filter(|r| r.estado == EstadoReporte::Pendiente)
        .count();
    let fallidos = reportes
        .iter()
        .filter(|r| r.estado == EstadoReporte::Fallido)
        .count();

    let reportes_response: Vec<ReporteResponse> =
        reportes.into_iter().map(ReporteResponse::from).collect();

    Ok(ReporteListResponse {
        reportes: reportes_response,
        total,
        enviados,
        pendientes,
        fallidos,
    })
}

// ==========================================
// OBTENER REPORTE POR ID
// ==========================================

pub async fn get_reporte_by_id(pool: &SqlitePool, id: &str) -> Result<ReporteResponse, String> {
    let reporte = db::find_by_id(pool, id).await?;
    Ok(ReporteResponse::from(reporte))
}

// ==========================================
// OBTENER REPORTES POR TIPO
// ==========================================

pub async fn get_reportes_by_tipo(
    pool: &SqlitePool,
    tipo: &str,
) -> Result<Vec<ReporteResponse>, String> {
    // Validar tipo
    TipoReporte::from_str(tipo)
        .ok_or_else(|| "Tipo invalido. Use: error, sugerencia, mejora".to_string())?;

    let reportes = db::find_by_tipo(pool, tipo).await?;
    Ok(reportes.into_iter().map(ReporteResponse::from).collect())
}

// ==========================================
// REINTENTAR ENVIO
// ==========================================

pub async fn reintentar_envio(pool: &SqlitePool, id: &str) -> Result<ReporteResponse, String> {
    // 1. Obtener reporte existente
    let reporte = db::find_by_id(pool, id).await?;

    // 2. Solo se puede reintentar si esta fallido
    if reporte.estado != EstadoReporte::Fallido {
        return Err("Solo se pueden reintentar reportes con estado 'fallido'".to_string());
    }

    // 3. Intentar enviar nuevamente
    // Nota: No tenemos el adjunto guardado, solo sabemos si tenia uno
    let resultado_envio =
        enviar_email(&reporte.asunto, &reporte.mensaje, reporte.contacto.as_deref(), None, None).await;

    // 4. Actualizar estado
    let now = Utc::now().to_rfc3339();
    match resultado_envio {
        Ok(_) => {
            db::update_estado(
                pool,
                id,
                EstadoReporte::Enviado.as_str(),
                None,
                Some(&now),
                &now,
            )
            .await?;
        }
        Err(ref e) => {
            db::update_estado(
                pool,
                id,
                EstadoReporte::Fallido.as_str(),
                Some(e),
                None,
                &now,
            )
            .await?;
            return Err(format!("Reintento fallido: {}", e));
        }
    }

    let reporte_actualizado = db::find_by_id(pool, id).await?;
    Ok(ReporteResponse::from(reporte_actualizado))
}

// ==========================================
// FUNCION INTERNA: ENVIAR EMAIL
// ==========================================

async fn enviar_email(
    asunto: &str,
    mensaje: &str,
    contacto: Option<&str>,
    adjunto_base64: Option<&str>,
    nombre_adjunto: Option<&str>,
) -> Result<(), String> {
    // 1. Cargar variables de entorno
    let smtp_host = env::var("SMTP_HOST").map_err(|_| "SMTP_HOST no configurado".to_string())?;
    let smtp_port_str = env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string());
    let smtp_port: u16 = smtp_port_str
        .parse()
        .map_err(|_| "SMTP_PORT invalido".to_string())?;
    let smtp_user = env::var("SMTP_USER").map_err(|_| "SMTP_USER no configurado".to_string())?;
    let smtp_pass =
        env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD no configurado".to_string())?;
    let feedback_email =
        env::var("FEEDBACK_EMAIL").map_err(|_| "FEEDBACK_EMAIL no configurado".to_string())?;

    // 2. Formatear cuerpo del email
    let contacto_str = contacto.unwrap_or("Anonimo");
    let email_body = format!(
        "{} de Brisas App\n\nDe: {}\n\nMensaje:\n{}",
        asunto, contacto_str, mensaje
    );

    // 3. Construir mensaje
    let email_builder = Message::builder()
        .from(
            format!("Brisas App <{}>", smtp_user)
                .parse::<lettre::message::Mailbox>()
                .map_err(|e| e.to_string())?,
        )
        .to(feedback_email
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| e.to_string())?)
        .subject(format!("Brisas App: {}", asunto));

    let email = if let (Some(base64_data), Some(filename)) = (adjunto_base64, nombre_adjunto) {
        // Decodificar base64
        let image_data =
            STANDARD.decode(base64_data).map_err(|e| format!("Error Base64: {}", e))?;

        // Detectar tipo MIME
        let content_type = if filename.ends_with(".png") {
            ContentType::parse("image/png").unwrap()
        } else if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
            ContentType::parse("image/jpeg").unwrap()
        } else {
            ContentType::parse("application/octet-stream").unwrap()
        };

        let attachment = Attachment::new(filename.to_string()).body(image_data, content_type);

        email_builder
            .multipart(
                MultiPart::mixed()
                    .singlepart(SinglePart::plain(email_body))
                    .singlepart(attachment),
            )
            .map_err(|e| e.to_string())?
    } else {
        // Solo texto plano
        email_builder
            .header(ContentType::TEXT_PLAIN)
            .body(email_body)
            .map_err(|e| e.to_string())?
    };

    // 4. Configurar transporte SMTP
    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::relay(&smtp_host)
        .map_err(|e| e.to_string())?
        .port(smtp_port)
        .credentials(creds)
        .build();

    // 5. Enviar
    mailer
        .send(&email)
        .map_err(|e| format!("Error al enviar email: {}", e))?;

    Ok(())
}
