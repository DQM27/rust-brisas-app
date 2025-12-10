// src/lib/stores/agGridSettings.svelte.ts

import type {
  AGGridTheme,
  AGGridFont,
  GridId,
  GridConfiguration,
  RowHeight,
  ToolbarButtonsConfig,
  AGGridColumnConfig,
  ConfirmationsConfig,
  ToolbarContext
} from '$lib/types/agGrid';

// ============================================
// Tipos Internos
// ============================================

interface AGGridGlobalSettings {
  configurations: Map<GridId, GridConfiguration>;
  defaultTheme: AGGridTheme;
  defaultFont: AGGridFont;
  buttonLimits: {
    default: number;
    singleSelect: number;
    multiSelect: number;
  };
}

// ============================================
// Valores por Defecto
// ============================================

const DEFAULT_CONFIG: Omit<GridConfiguration, 'gridId'> = {
  // Apariencia
  theme: 'ag-theme-quartz-dark',
  font: 'system',
  rowHeight: 'normal',
  headerHeight: 40,
  animateRows: true,
  enableCellTextSelection: true,

  // Columnas
  columns: [],

  // Botones
  buttons: {
    default: { order: [], hidden: [] },
    singleSelect: { order: [], hidden: [] },
    multiSelect: { order: [], hidden: [] }
  },

  // Datos
  paginationSize: 50,
  showFloatingFilters: false,
  enableQuickFilter: false,

  // Features
  enableGrouping: false,
  enableFilters: true,
  enableSidebar: false,
  enableUndoRedo: false,

  // Performance
  rowBuffer: 10,
  debounceVerticalScrollbar: true,

  // Confirmaciones
  confirmations: {
    deleteRecords: true,
    bulkOperations: true,
    dontAskAgain: false
  }
};

// ============================================
// Store Class
// ============================================

class AGGridSettingsStore {
  private settings = $state<AGGridGlobalSettings>({
    configurations: new Map(),
    defaultTheme: 'ag-theme-quartz-dark',
    defaultFont: 'system',
    buttonLimits: {
      default: 8,
      singleSelect: 6,
      multiSelect: 5
    }
  });

  private version = $state(0);

  constructor() {
    this.loadFromStorage();
  }

  private triggerUpdate() {
    this.version++;
  }

  // ============================================
  // Configuración Base
  // ============================================

  getConfiguration(gridId: GridId): GridConfiguration | null {
    this.version;
    return this.settings.configurations.get(gridId) ?? null;
  }

  getOrCreateConfiguration(gridId: GridId): GridConfiguration {
    this.version;
    const existing = this.settings.configurations.get(gridId);
    if (existing) return existing;

    const defaultConfig: GridConfiguration = {
      ...DEFAULT_CONFIG,
      gridId
    };

    this.settings.configurations.set(gridId, defaultConfig);
    this.saveToStorage();
    this.triggerUpdate();
    return defaultConfig;
  }

  private updateConfig(gridId: GridId, updates: Partial<GridConfiguration>): void {
    const config = this.getOrCreateConfiguration(gridId);
    const newConfig = { ...config, ...updates };
    this.settings.configurations.set(gridId, newConfig);
    this.saveToStorage();
    this.triggerUpdate();
  }

  // ============================================
  // Apariencia
  // ============================================

  getTheme(gridId: GridId): AGGridTheme {
    this.version;
    return this.getConfiguration(gridId)?.theme ?? this.settings.defaultTheme;
  }

  setTheme(gridId: GridId, theme: AGGridTheme): void {
    this.updateConfig(gridId, { theme });
  }

  getFont(gridId: GridId): AGGridFont {
    this.version;
    return this.getConfiguration(gridId)?.font ?? this.settings.defaultFont;
  }

  setFont(gridId: GridId, font: AGGridFont): void {
    this.updateConfig(gridId, { font });
  }

  getRowHeight(gridId: GridId): RowHeight {
    this.version;
    return this.getConfiguration(gridId)?.rowHeight ?? 'normal';
  }

  setRowHeight(gridId: GridId, height: RowHeight): void {
    this.updateConfig(gridId, { rowHeight: height });
  }

  getHeaderHeight(gridId: GridId): number {
    this.version;
    return this.getConfiguration(gridId)?.headerHeight ?? 40;
  }

  setHeaderHeight(gridId: GridId, height: number): void {
    this.updateConfig(gridId, { headerHeight: height });
  }

  getAnimateRows(gridId: GridId): boolean {
    this.version;
    return this.getConfiguration(gridId)?.animateRows ?? true;
  }

  setAnimateRows(gridId: GridId, animate: boolean): void {
    this.updateConfig(gridId, { animateRows: animate });
  }

  getCellTextSelection(gridId: GridId): boolean {
    this.version;
    return this.getConfiguration(gridId)?.enableCellTextSelection ?? true;
  }

  setCellTextSelection(gridId: GridId, enable: boolean): void {
    this.updateConfig(gridId, { enableCellTextSelection: enable });
  }

  getThemeClass(gridId: GridId): string {
    return this.getTheme(gridId);
  }

  getFontClass(gridId: GridId): string {
    const font = this.getFont(gridId);
    const fontMap: Record<AGGridFont, string> = {
      'system': 'font-system',
      'inter': 'font-inter',
      'roboto': 'font-roboto',
      'source-sans': 'font-source-sans'
    };
    return fontMap[font];
  }

  getRowHeightPx(gridId: GridId): number {
    const height = this.getRowHeight(gridId);
    const heightMap: Record<RowHeight, number> = {
      'compact': 32,
      'normal': 40,
      'comfortable': 48
    };
    return heightMap[height];
  }

  // ============================================
  // Columnas
  // ============================================

  getColumnsConfig(gridId: GridId): AGGridColumnConfig[] {
    this.version;
    return this.getConfiguration(gridId)?.columns ?? [];
  }

  setColumnsConfig(gridId: GridId, columns: AGGridColumnConfig[]): void {
    this.updateConfig(gridId, { columns });
  }

  // ============================================
  // Botones de Toolbar
  // ============================================

  getButtonsConfig(gridId: GridId, context: ToolbarContext): ToolbarButtonsConfig {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.buttons[context] ?? { order: [], hidden: [] };
  }

  setButtonOrder(gridId: GridId, context: ToolbarContext, order: string[]): void {
    const config = this.getOrCreateConfiguration(gridId);
    this.updateConfig(gridId, {
      buttons: {
        ...config.buttons,
        [context]: { ...config.buttons[context], order }
      }
    });
  }

  setHiddenButtons(gridId: GridId, context: ToolbarContext, hidden: string[]): void {
    const config = this.getOrCreateConfiguration(gridId);
    this.updateConfig(gridId, {
      buttons: {
        ...config.buttons,
        [context]: { ...config.buttons[context], hidden }
      }
    });
  }

  getButtonLimit(context: ToolbarContext): number {
    return this.settings.buttonLimits[context];
  }

  // ============================================
  // Datos y Filtros
  // ============================================

  getPaginationSize(gridId: GridId): number {
    this.version;
    return this.getConfiguration(gridId)?.paginationSize ?? 50;
  }

  setPaginationSize(gridId: GridId, size: number): void {
    this.updateConfig(gridId, { paginationSize: size });
  }

  getShowFloatingFilters(gridId: GridId): boolean {
    this.version;
    return this.getConfiguration(gridId)?.showFloatingFilters ?? false;
  }

  setShowFloatingFilters(gridId: GridId, show: boolean): void {
    this.updateConfig(gridId, { showFloatingFilters: show });
  }

  getEnableQuickFilter(gridId: GridId): boolean {
    this.version;
    return this.getConfiguration(gridId)?.enableQuickFilter ?? false;
  }

  setEnableQuickFilter(gridId: GridId, enable: boolean): void {
    this.updateConfig(gridId, { enableQuickFilter: enable });
  }

  // ============================================
  // Features
  // ============================================

  getEnableUndoRedo(gridId: GridId): boolean {
    this.version;
    return this.getConfiguration(gridId)?.enableUndoRedo ?? false;
  }

  setEnableUndoRedo(gridId: GridId, enable: boolean): void {
    this.updateConfig(gridId, { enableUndoRedo: enable });
  }

  // ============================================
  // Performance
  // ============================================

  getRowBuffer(gridId: GridId): number {
    this.version;
    return this.getConfiguration(gridId)?.rowBuffer ?? 10;
  }

  setRowBuffer(gridId: GridId, buffer: number): void {
    this.updateConfig(gridId, { rowBuffer: buffer });
  }

  getDebounceScroll(gridId: GridId): boolean {
    this.version;
    return this.getConfiguration(gridId)?.debounceVerticalScrollbar ?? true;
  }

  setDebounceScroll(gridId: GridId, debounce: boolean): void {
    this.updateConfig(gridId, { debounceVerticalScrollbar: debounce });
  }

  // ============================================
  // Confirmaciones
  // ============================================

  getConfirmations(gridId: GridId): ConfirmationsConfig {
    this.version;
    return this.getConfiguration(gridId)?.confirmations ?? {
      deleteRecords: true,
      bulkOperations: true,
      dontAskAgain: false
    };
  }

  setConfirmations(gridId: GridId, confirmations: ConfirmationsConfig): void {
    this.updateConfig(gridId, { confirmations });
  }

  // ============================================
  // Reset
  // ============================================

  resetConfiguration(gridId: GridId): void {
    this.settings.configurations.delete(gridId);
    this.saveToStorage();
    this.triggerUpdate();
  }

  resetToDefaults(gridId: GridId): void {
    const defaultConfig: GridConfiguration = {
      ...DEFAULT_CONFIG,
      gridId
    };
    this.settings.configurations.set(gridId, defaultConfig);
    this.saveToStorage();
    this.triggerUpdate();
  }

  // ============================================
  // Persistencia
  // ============================================

  private loadFromStorage(): void {
    if (typeof localStorage === 'undefined') return;

    const stored = localStorage.getItem('agGridSettings');
    if (stored) {
      try {
        const parsed = JSON.parse(stored);

        if (parsed.configurations) {
          const entries = Object.entries(parsed.configurations) as [GridId, GridConfiguration][];
          this.settings.configurations = new Map(entries);
        }

        if (parsed.defaultTheme) {
          this.settings.defaultTheme = parsed.defaultTheme;
        }

        if (parsed.defaultFont) {
          this.settings.defaultFont = parsed.defaultFont;
        }

        if (parsed.buttonLimits) {
          this.settings.buttonLimits = parsed.buttonLimits;
        }
      } catch (e) {
        console.error('Error loading AG Grid settings:', e);
      }
    }
  }

  private saveToStorage(): void {
    if (typeof localStorage === 'undefined') return;

    const toSave = {
      configurations: Object.fromEntries(this.settings.configurations),
      defaultTheme: this.settings.defaultTheme,
      defaultFont: this.settings.defaultFont,
      buttonLimits: this.settings.buttonLimits
    };
    localStorage.setItem('agGridSettings', JSON.stringify(toSave));
  }

  // ============================================
  // Export/Import
  // ============================================

  exportSettings(gridId: GridId): string {
    const config = this.getConfiguration(gridId);
    return JSON.stringify(config, null, 2);
  }

  importSettings(gridId: GridId, json: string): boolean {
    try {
      const config = JSON.parse(json) as GridConfiguration;
      config.gridId = gridId;
      this.settings.configurations.set(gridId, config);
      this.saveToStorage();
      this.triggerUpdate();
      return true;
    } catch {
      return false;
    }
  }
}

export const agGridSettings = new AGGridSettingsStore();
