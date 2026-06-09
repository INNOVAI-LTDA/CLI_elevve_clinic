/**
 * User - modelo unificado de usuário do Elevve Clinic.
 *
 * F2 refactor: substitui o modelo antigo que distinguia 'mentor' e
 * 'aluno' como entidades separadas. Agora existe apenas User, com
 * UserType (Admin | Provider | Client).
 *
 * UserType mapeia 1:1 com o campo `role` em deva_elevveclinic_users
 * (que ja tem CHECK IN ('admin', 'provider', 'client')).
 *
 * - Admin: administrador do sistema
 * - Provider: mentor / medico
 * - Client: aluno / paciente
 */

export type UserType = "admin" | "provider" | "client";

/** Lista utilitaria de todos os UserTypes (ordem deterministica). */
export const USER_TYPES: readonly UserType[] = ["admin", "provider", "client"] as const;

export type User = {
  id: number | string;
  email: string;
  userType: UserType;
  fullName?: string;
  organizationId?: number | null;
};

/**
 * Normaliza uma role crua (string vinda do backend) para UserType.
 * Aceita tambem os termos legados 'mentor' e 'aluno' para
 * nao quebrar sessoes de localStorage de versoes anteriores.
 * Retorna null se a role nao e' reconhecida.
 */
export function normalizeUserType(raw: string | null | undefined): UserType | null {
  if (raw === "admin" || raw === "provider" || raw === "client") {
    return raw;
  }
  if (raw === "mentor") return "provider";
  if (raw === "aluno") return "client";
  return null;
}
