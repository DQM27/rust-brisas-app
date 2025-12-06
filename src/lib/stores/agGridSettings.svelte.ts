// src/lib/stores/agGridSettings.svelte.ts

import type {
  AGGridTheme,
  AGGridFont,
  GridId,
  GridConfiguration,
  RowHeight,
  ToolbarButtonsConfig,
  AGGridColumnConfig,
  ConfirmationsConfig
} from '$lib/types/agGrid';

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

  // NUEVO: Versión para forzar reactividad
  private version = $state(0);

  constructor() {
    this.loadFromStorage();
  }

  // NUEVO: Helper para forzar actualización
  private triggerUpdate() {
    this.version++;
  }

  // ============================================
  // Getters de Configuración
  // ============================================

  getConfiguration(gridId: GridId): GridConfiguration | null {
    // Usar version para forzar reactividad
    this.version;
    return this.settings.configurations.get(gridId) ?? null;
  }

  getOrCreateConfiguration(gridId: GridId): GridConfiguration {
    this.version;
    const existing = this.settings.configurations.get(gridId);
    if (existing) return existing;

    const defaultConfig: GridConfiguration = {
      gridId,
      theme: this.settings.defaultTheme,
      font: this.settings.defaultFont,
      columns: [],
      buttons: {
        default: { order: [], hidden: [] },
        singleSelect: { order: [], hidden: [] },
        multiSelect: { order: [], hidden: [] }
      },
      rowHeight: 'normal',
      paginationSize: 50,
      enableGrouping: false,
      enableFilters: true,
      enableSidebar: false,
      confirmations: {
        deleteRecords: true,
        dontAskAgain: false
      },
      showFloatingFilters: false
    };

    this.settings.configurations.set(gridId, defaultConfig);
    this.saveToStorage();
    this.triggerUpdate();
    return defaultConfig;
  }

  // ============================================
  // Visual Settings
  // ============================================

  getTheme(gridId: GridId): AGGridTheme {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.theme ?? this.settings.defaultTheme;
  }

  setTheme(gridId: GridId, theme: AGGridTheme): void {
    const config = this.getOrCreateConfiguration(gridId);

    // Crear nuevo objeto para forzar reactividad
    const newConfig = { ...config, theme };
    this.settings.configurations.set(gridId, newConfig);

    this.saveToStorage();
    this.triggerUpdate();
  }

  getFont(gridId: GridId): AGGridFont {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.font ?? this.settings.defaultFont;
  }

  setFont(gridId: GridId, font: AGGridFont): void {
    const config = this.getOrCreateConfiguration(gridId);

    const newConfig = { ...config, font };
    this.settings.configurations.set(gridId, newConfig);

    this.saveToStorage();
    this.triggerUpdate();
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

  // ============================================
  // Buttons Settings
  // ============================================

  getButtonsConfig(
    gridId: GridId,
    context: 'default' | 'singleSelect' | 'multiSelect'
  ): ToolbarButtonsConfig {
    this.version;
    const config = this.getConfiguration(gridId);
    if (!config) {
      return { order: [], hidden: [] };
    }
    return config.buttons[context];
  }

  setButtonOrder(
    gridId: GridId,
    context: 'default' | 'singleSelect' | 'multiSelect',
    order: string[]
  ): void {
    const config = this.getOrCreateConfiguration(gridId);

    // Crear nuevo objeto para forzar reactividad
    const newConfig = {
      ...config,
      buttons: {
        ...config.buttons,
        [context]: {
          ...config.buttons[context],
          order
        }
      }
    };

    this.settings.configurations.set(gridId, newConfig);
    this.saveToStorage();
    this.triggerUpdate();
  }

  setHiddenButtons(
    gridId: GridId,
    context: 'default' | 'singleSelect' | 'multiSelect',
    hidden: string[]
  ): void {
    const config = this.getOrCreateConfiguration(gridId);

    // Crear nuevo objeto para forzar reactividad
    const newConfig = {
      ...config,
      buttons: {
        ...config.buttons,
        [context]: {
          ...config.buttons[context],
          hidden
        }
      }
    };

    this.settings.configurations.set(gridId, newConfig);
    this.saveToStorage();
    this.triggerUpdate();
  }

  getButtonLimit(context: 'default' | 'singleSelect' | 'multiSelect'): number {
    return this.settings.buttonLimits[context];
  }

  countVisibleButtons(
    gridId: GridId,
    context: 'default' | 'singleSelect' | 'multiSelect'
  ): number {
    const config = this.getButtonsConfig(gridId, context);
    return config.order.filter(id => !config.hidden.includes(id)).length;
  }

  // ============================================
  // Columns Settings
  // ============================================

  getColumnsConfig(gridId: GridId): AGGridColumnConfig[] {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.columns ?? [];
  }

  setColumnsConfig(gridId: GridId, columns: AGGridColumnConfig[]): void {
    const config = this.getOrCreateConfiguration(gridId);

    const newConfig = { ...config, columns };
    this.settings.configurations.set(gridId, newConfig);

    this.saveToStorage();
    this.triggerUpdate();
  }

  // ============================================
  // Advanced Settings
  // ============================================

  getRowHeight(gridId: GridId): RowHeight {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.rowHeight ?? 'normal';
  }

  setRowHeight(gridId: GridId, height: RowHeight): void {
    const config = this.getOrCreateConfiguration(gridId);

    const newConfig = { ...config, rowHeight: height };
    this.settings.configurations.set(gridId, newConfig);

    this.saveToStorage();
    this.triggerUpdate();
  }

  getPaginationSize(gridId: GridId): number {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.paginationSize ?? 50;
  }

  setPaginationSize(gridId: GridId, size: number): void {
    const config = this.getOrCreateConfiguration(gridId);

    const newConfig = { ...config, paginationSize: size };
    this.settings.configurations.set(gridId, newConfig);

    this.saveToStorage();
    this.triggerUpdate();
  }

  getConfirmations(gridId: GridId): ConfirmationsConfig {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.confirmations ?? { deleteRecords: true, dontAskAgain: false };
  }

  setConfirmations(gridId: GridId, confirmations: ConfirmationsConfig): void {
    const config = this.getOrCreateConfiguration(gridId);

    const newConfig = { ...config, confirmations };
    this.settings.configurations.set(gridId, newConfig);

    this.saveToStorage();
    this.triggerUpdate();
  }

  // ============================================
  // Reset & Persistence
  // ============================================

  getShowFloatingFilters(gridId: GridId): boolean {
    this.version;
    const config = this.getConfiguration(gridId);
    return config?.showFloatingFilters ?? false;
  }

  setShowFloatingFilters(gridId: GridId, show: boolean): void {
    const config = this.getOrCreateConfiguration(gridId);
    const newConfig = { ...config, showFloatingFilters: show };
    this.settings.configurations.set(gridId, newConfig);
    this.saveToStorage();
    this.triggerUpdate();
  }

  resetConfiguration(gridId: GridId): void {
    this.settings.configurations.delete(gridId);
    this.saveToStorage();
    this.triggerUpdate();
  }

  private loadFromStorage(): void {
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
    const toSave = {
      configurations: Object.fromEntries(this.settings.configurations),
      defaultTheme: this.settings.defaultTheme,
      defaultFont: this.settings.defaultFont,
      buttonLimits: this.settings.buttonLimits
    };
    localStorage.setItem('agGridSettings', JSON.stringify(toSave));
  }
}

export const agGridSettings = new AGGridSettingsStore();