import type { UserType } from "../../contracts/user";

/**
 * F2 refactor: substitui o modelo antigo que distinguia 'mentor' e
 * 'aluno' como tipos diferentes. Agora existe apenas UserType
 * (Admin | Provider | Client).
 *
 * Cada UserType redireciona para a home do BI Elevve (unico produto
 * ativo no frontend). 'admin' tem label especial.
 */

export type KnownUserRole = UserType;

export function isKnownUserRole(
  role: string | null | undefined
): role is KnownUserRole {
  return role === "admin" || role === "provider" || role === "client";
}

export function getDefaultRouteForRole(role: KnownUserRole): string {
  // F2: todos os tipos vao para o BI (unico produto)
  switch (role) {
    case "admin":
    case "provider":
    case "client":
      return "/app/bi/radar";
  }
}

export function getRoleHomeLabel(role: KnownUserRole): string {
  switch (role) {
    case "admin":
      return "Ir para a area administrativa";
    case "provider":
      return "Ir para o BI Elevve (Provider)";
    case "client":
      return "Ir para a area do cliente";
  }
}
