import type { LoginRequestDto, LoginResponseDto, MeResponseDto } from "../../contracts/auth";
import { httpClient } from "../../shared/api/httpClient";
import { AppError } from "../../shared/api/types";
import { clearAccessToken, getAccessToken, setAccessToken } from "../../shared/auth/tokenStorage";
import { isKnownUserRole } from "../../shared/auth/roleRouting";
import { normalizeUserType } from "../../contracts/user";
import type { AuthSession } from "../models";

/**
 * F2 refactor: o `me.role` retornado pelo backend e' uma string
 * crua (admin | provider | client ou termo legado). Aqui normalizamos
 * para UserType. Se nao for reconhecido, lancamos AUTH_ROLE_INVALID.
 */
function normalizeAuthenticatedUser(me: MeResponseDto): NonNullable<AuthSession["user"]> {
  const userType = normalizeUserType(me.role);
  if (!userType || !isKnownUserRole(userType)) {
    throw new AppError({
      message: "Perfil da conta nao reconhecido.",
      code: "AUTH_ROLE_INVALID",
    });
  }

  return {
    id: me.id,
    email: me.email,
    userType,
  };
}

export async function login(credentials: LoginRequestDto): Promise<AuthSession> {
  const payload = await httpClient.post<LoginResponseDto>(
    "/auth/login",
    credentials,
    { token: null }
  );
  if (payload.access_token) {
    setAccessToken(payload.access_token);
  }

  try {
    const me = await httpClient.get<MeResponseDto>("/me");
    return {
      accessToken: payload.access_token,
      tokenType: payload.token_type,
      user: normalizeAuthenticatedUser(me),
    };
  } catch (error) {
    if (error instanceof AppError && error.isNetworkError) {
      throw new AppError({
        message:
          "Autenticacao concluida, mas nao foi possivel validar seu perfil. Tente novamente.",
        code: "AUTH_BOOTSTRAP_RETRYABLE",
        isNetworkError: true,
        details: {
          accessToken: getAccessToken(),
        },
      });
    }
    clearAccessToken();
    throw error;
  }
}

export async function getMe() {
  const me = await httpClient.get<MeResponseDto>("/me");
  return normalizeAuthenticatedUser(me);
}

export function logout() {
  clearAccessToken();
}
