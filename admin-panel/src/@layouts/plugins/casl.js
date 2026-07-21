/**
 * v1 has no fine-grained permission gating: any authenticated user sees every
 * module (each backend service still enforces its own auth independently).
 * These stubs keep the existing nav/guard call sites working without a CASL
 * dependency; swap in real ability checks here if per-role gating is added later.
 */
export const can = () => true

export const canViewNavMenuGroup = () => true

export const canNavigate = () => true
