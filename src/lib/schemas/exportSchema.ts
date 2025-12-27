import { z } from "zod";

// --- Validaciones de Tipos Base ---
export const MeasureUnitSchema = z.enum(["mm", "cm", "in", "pt"]);

export const PdfOrientationSchema = z.enum(["portrait", "landscape"]);

export const PageSizeSchema = z.enum(["us-letter", "a4", "legal"]);

export const PdfColorsSchema = z.object({
    header_fill: z.string(),
    header_text: z.string(),
    row_text: z.string(),
    border: z.string(),
});

export const PdfFontsSchema = z.object({
    family: z.string(),
    size: z.number().min(6).max(72),
    header_size: z.number().min(6).max(72),
});

export const PdfDesignSchema = z.object({
    page_size: PageSizeSchema,
    orientation: PdfOrientationSchema,
    margin_x: z.number().min(0),
    margin_x_unit: MeasureUnitSchema,
    margin_y: z.number().min(0),
    margin_y_unit: MeasureUnitSchema,
    colors: PdfColorsSchema,
    fonts: PdfFontsSchema,
});

export const CsvDelimiterSchema = z.enum(["comma", "semicolon", "tab", "pipe"]);

export const CsvOptionsSchema = z.object({
    delimiter: CsvDelimiterSchema,
    include_bom: z.boolean(),
});

// --- Export Profile Schema ---
export const ExportProfileSchema = z.object({
    id: z.string().uuid(),
    name: z.string().min(1, "El nombre es requerido"),
    format: z.enum(["pdf", "excel", "csv"]),
    is_default: z.boolean(),
    title: z.string().optional(),
    show_preview: z.boolean().optional(),
    pdf_design: PdfDesignSchema.optional(),
    csv_options: CsvOptionsSchema.optional(),
});

// --- Export Request Schema (Para el backend) ---
export const ExportRequestSchema = z.object({
    format: z.enum(['pdf', 'excel', 'csv']),
    headers: z.array(z.string()),
    rows: z.array(z.record(z.string(), z.any())),
    title: z.string().optional(),
    orientation: PdfOrientationSchema.optional(),
    showPreview: z.boolean().optional(),
    templateId: z.string().optional(),
    fontSize: z.number().optional(),
    fontFamily: z.string().optional(),
    marginTop: z.number().optional(),
    marginBottom: z.number().optional(),
    marginLeft: z.number().optional(),
    marginRight: z.number().optional(),
    bannerColor: z.string().optional(),
    delimiter: CsvDelimiterSchema.optional(),
    includeBom: z.boolean().optional(),
    targetPath: z.string().optional(),
    generatedBy: z.string().optional(),
});

// --- Export Options Schema (Frontend state) ---
export const ExportOptionsSchema = z.object({
    title: z.string().optional(),
    orientation: PdfOrientationSchema.optional(),
    delimiter: CsvDelimiterSchema.optional(),
    includeBom: z.boolean().optional(),
    showPreview: z.boolean().optional(),
    templateId: z.string().optional(),
    columnIds: z.array(z.string()).optional(),
    fontSize: z.number().optional(),
    fontFamily: z.string().optional(),
    marginTop: z.number().optional(),
    marginBottom: z.number().optional(),
    marginLeft: z.number().optional(),
    marginRight: z.number().optional(),
    bannerColor: z.string().optional(),
    generatedBy: z.string().optional(),
});

// Types inferred from schema
export type ExportProfileInput = z.infer<typeof ExportProfileSchema>;
export type ExportRequestInput = z.infer<typeof ExportRequestSchema>;
export type ExportOptionsInput = z.infer<typeof ExportOptionsSchema>;
