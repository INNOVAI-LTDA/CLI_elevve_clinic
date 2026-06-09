import type { UserType } from "./user";

/**
 * F2 refactor: o backend continua retornando `role: string` no
 * MeResponseDto (espelha o campo `role` em deva_elevveclinic_users
 * que ja tem CHECK IN ('admin', 'provider', 'client')). O frontend
 * normaliza essa string para UserType via normalizeUserType().
 *
 * Manter o campo `role` no DTO evita mudar o backend nesta fase.
 */

export type LoginRequestDto = {
  email: string;
  password: string;
};

export type LoginResponseDto = {
  access_token: string;
  token_type: string;
};

export type MeResponseDto = {
  id: number | string;
  email: string;
  role: UserType | string;
};
