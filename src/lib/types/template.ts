// src/lib/types/template.ts

export interface PdfTemplate {
    id: string;
    name: string;
    is_predefined: boolean;
    colors: TemplateColors;
    fonts: TemplateFonts;
    layout: TemplateLayout;
}

export interface TemplateColors {
    header_fill: string;
    header_text: string;
    row_text: string;
    border: string;
}

export interface TemplateFonts {
    family: string;
    size: number;
    header_size: number;
}

export interface TemplateLayout {
    page_size: string;
    orientation: string;
    margin_x: string;
    margin_y: string;
}

export const DEFAULT_TEMPLATE: PdfTemplate = {
    id: "new-custom",
    name: "Nuevo Estilo",
    is_predefined: false,
    colors: {
        header_fill: "#e8e8e8",
        header_text: "#000000",
        row_text: "#000000",
        border: "#000000"
    },
    fonts: {
        family: "New Computer Modern",
        size: 10,
        header_size: 11
    },
    layout: {
        page_size: "us-letter",
        orientation: "landscape",
        margin_x: "1.5cm",
        margin_y: "2cm"
    }
};
