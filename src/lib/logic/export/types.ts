export interface ExportRequest {
    format: 'pdf' | 'excel' | 'csv';
    headers: string[];
    rows: Record<string, any>[];

    // Opcionales para PDF
    title?: string;
    orientation?: 'portrait' | 'landscape';
    showPreview?: boolean;
    templateId?: string;
    fontSize?: number;
    fontFamily?: string;
    marginTop?: number;
    marginBottom?: number;
    marginLeft?: number;
    marginRight?: number;
    bannerColor?: string;

    // Opcionales para CSV
    delimiter?: 'comma' | 'semicolon' | 'tab' | 'pipe';
    includeBom?: boolean;

    // Opcionales generales
    targetPath?: string;
}

export interface ExportResponse {
    success: boolean;
    format: string;
    bytes?: number[];
    filePath?: string;
    message: string;
}

export interface ExportOptions {
    title?: string;
    orientation?: 'portrait' | 'landscape';
    delimiter?: 'comma' | 'semicolon' | 'tab' | 'pipe';
    includeBom?: boolean;
    showPreview?: boolean;
    templateId?: string;
    columnIds?: string[];
    fontSize?: number;
    fontFamily?: string;
    marginTop?: number;
    marginBottom?: number;
    marginLeft?: number;
    marginRight?: number;
    bannerColor?: string;
}




