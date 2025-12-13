// src/lib/types/exportProfile.ts
export interface ExportProfile {
  id: string;
  name: string;
  format: "pdf" | "excel" | "csv";
  is_default: boolean;
  options: {
    title?: string;
    orientation?: "portrait" | "landscape";
    template_id?: string;
    delimiter?: "comma" | "semicolon" | "tab" | "pipe";
    include_bom?: boolean;
    show_preview?: boolean;
  };
}

export interface ExportProfileStore {
  profiles: ExportProfile[];
  loading: boolean;
}
