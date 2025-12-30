// ============================================
// src/lib/config/demoUsers.ts
// ============================================
// Demo user data for development/testing mode

export interface DemoUser {
    email: string;
    role: string;
    name: string;
    icon: string;
}

/**
 * Demo users inspired by famous scientists
 * Used in LoginForm when showDemoLink is enabled
 */
export const DEMO_USERS: DemoUser[] = [
    {
        email: "marie.curie@demo.com",
        role: "Supervisora",
        name: "Marie Curie",
        icon: "👩‍🔬",
    },
    {
        email: "albert.einstein@demo.com",
        role: "Administrador",
        name: "Albert Einstein",
        icon: "👨‍🔬",
    },
    {
        email: "richard.feynman@demo.com",
        role: "Guardia",
        name: "Richard Feynman",
        icon: "🧑‍🔬",
    },
];
