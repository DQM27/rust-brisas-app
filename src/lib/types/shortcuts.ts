export interface ShortcutConfig {
	version: number;
	global: Record<string, string>;
	contexts: Record<string, Record<string, string>>;
}

export type CommandHandler = (args?: any) => void;

export interface ShortcutContext {
	id: string;
	active: boolean;
}
