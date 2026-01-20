/**
 * Spotlight Search Types
 */

export type SpotlightCategory = 'module' | 'action' | 'tab';
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
}

export interface SpotlightGroups {
    modules: SpotlightItem[];
    actions: SpotlightItem[];
    tabs: SpotlightItem[];
}
