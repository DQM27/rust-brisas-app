// src/lib/types/exportProfile.ts

// Unidades de medida soportadas
export type MeasureUnit = "mm" | "cm" | "in" | "pt";

// Configuración de diseño PDF (integrada en perfil)
export interface PdfDesign {
  page_size: "us-letter" | "a4" | "legal";
  orientation: "portrait" | "landscape";
  margin_x: number;
  margin_x_unit: MeasureUnit;
  margin_y: number;
  margin_y_unit: MeasureUnit;
  colors: {
    header_fill: string;
    header_text: string;
    row_text: string;
    border: string;
  };
  fonts: {
    family: string;
    size: number;
    header_size: number;
  };
}

// Perfil unificado (incluye TODO: formato + diseño + opciones)
export interface ExportProfile {
  id: string;
  name: string;
  format: "pdf" | "excel" | "csv";
  isDefault: boolean;

  // Opciones comunes
  title?: string;
  show_preview?: boolean;

  // Opciones PDF (incluye diseño completo)
  pdf_design?: PdfDesign;

  // Opciones CSV
  csv_options?: {
    delimiter: "comma" | "semicolon" | "tab" | "pipe";
    include_bom: boolean;
  };
}

export interface ExportProfileStore {
  profiles: ExportProfile[];
  loading: boolean;
}

// Perfil por defecto
export const DEFAULT_PDF_DESIGN: PdfDesign = {
  page_size: "us-letter",
  orientation: "landscape",
  margin_x: 15,
  margin_x_unit: "mm",
  margin_y: 20,
  margin_y_unit: "mm",
  colors: {
    header_fill: "#2da44e",
    header_text: "#ffffff",
    row_text: "#000000",
    border: "#cccccc",
  },
  fonts: {
    family: "New Computer Modern",
    size: 10,
    header_size: 12,
  },
};
