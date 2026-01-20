/**
 * Spotlight Search Types
 */

export type SpotlightCategory = 'module' | 'action' | 'tab' | 'data' | 'recent';
export type SpotlightSubCategory = 'master' | 'transaction' | 'settings' | 'link';

export interface SpotlightItem {
    id: string;
    label: string;
    description?: string;
    icon: any; // Componente Svelte (lucide-svelte)
    category: SpotlightCategory;
    subCategory?: SpotlightSubCategory;
    action: () => void;
    keywords?: string[];
    permission?: string;
    roleId?: string[];
    isOpen?: boolean;
    shortcut?: string;
}

export interface SpotlightItemDefinition {
    id: string;
    label: string;
    description?: string;
    icon: any; // Componente Svelte (lucide-svelte)
    category: SpotlightCategory;
    subCategory?: SpotlightSubCategory;
    keywords?: string[];
    permission?: string;
    roleId?: string[];
    shortcut?: string;
}

export interface SpotlightGroups {
    modules: SpotlightItem[];
    actions: SpotlightItem[];
    tabs: SpotlightItem[];
    data: SpotlightItem[];
    recent: SpotlightItem[];
}
