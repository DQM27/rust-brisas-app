// ============================================
// src/lib/logic/auth/loginValidation.ts
// ============================================
// Login form validation logic using Zod

import { LoginSchema, type LoginForm } from '$lib/schemas/userSchema';
import type { ZodIssue } from 'zod';

export type ValidationResult =
	| { valid: true; data: LoginForm }
	| { valid: false; errors: Record<string, string> };

/**
 * Maps Zod issues to a simple error record by field name
 */
function mapZodErrors(issues: ZodIssue[]): Record<string, string> {
	const errors: Record<string, string> = {};
	issues.forEach((issue) => {
		if (issue.path[0]) {
			errors[String(issue.path[0])] = issue.message;
		}
	});
	return errors;
}

/**
 * Validates login form data
 * @param data - The form data to validate
 * @returns Validation result with either valid data or errors by field
 */
export function validateLoginForm(email: string, password: string): ValidationResult {
	const result = LoginSchema.safeParse({ email, password });

	if (result.success) {
		return { valid: true, data: result.data };
	}

	return { valid: false, errors: mapZodErrors(result.error.issues) };
}
