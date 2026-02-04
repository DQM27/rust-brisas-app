// src/lib/types/keyring.ts

export interface CredentialStatus {
    argon2_configured: boolean;
    fully_configured: boolean;
}

export interface Argon2Params {
    memory: number;
    iterations: number;
    parallelism: number;
    secret: string;
}

export interface Argon2ParamsSafe {
    memory: number;
    iterations: number;
    parallelism: number;
    has_secret: boolean;
}

export interface SetupCredentialsInput {
    argon2: Argon2Params;
    terminal_name: string;
    terminal_location: string;
}

export interface SetupResult {
    success: boolean;
    message: string;
}
