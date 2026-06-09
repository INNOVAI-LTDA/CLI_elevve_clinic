import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { getMe, login as loginService, logout as logoutService } from "../../domain/services/authService";
import type { AuthSession } from "../../domain/models";
import { AppError, toUserErrorMessage } from "../../shared/api/types";
import { onUnauthorized } from "../../shared/auth/authEvents";
import { isKnownUserRole } from "../../shared/auth/roleRouting";
import { clearAccessToken, getAccessToken } from "../../shared/auth/tokenStorage";
import { env } from "../../shared/config/env";
import { normalizeUserType, type UserType } from "../../contracts/user";

/**
 * F2 refactor: PreviewRole agora e' UserType (admin | provider | client).
 * O auth real (login/getMe) continua retornando `role: string` do
 * backend; normalizamos para UserType via normalizeUserType().
 *
 * Sessoes legadas em localStorage com role 'mentor'/'aluno' sao
 * normalizadas em runtime: 'mentor' -> 'provider', 'aluno' -> 'client'.
 */

type PreviewRole = UserType;

type PreviewSession = {
  email: string;
  role: PreviewRole;
  token: string;
};

type AuthContextValue = {
  accessToken: string | null;
  user: AuthSession["user"];
  loading: boolean;
  error: string | null;
  errorCode: string | null;
  authReady: boolean;
  isAuthenticated: boolean;
  isPreviewSession: boolean;
  canUsePreviewLogin: boolean;
  sessionRecoveryPending: boolean;
  login: (email: string, password: string) => Promise<AuthSession["user"]>;
  loginPreview: (role: PreviewRole) => void;
  logout: () => void;
  refreshMe: () => Promise<void>;
  retrySessionValidation: () => Promise<void>;
  clearPendingSession: () => void;
};

const AuthContext = createContext<AuthContextValue | null>(null);
const PREVIEW_SESSION_KEY = "swaif_mvp_preview_session";

function hasLocalStorage(): boolean {
  return typeof localStorage !== "undefined";
}

function clearPreviewSession(): void {
  if (!hasLocalStorage()) {
    return;
  }
  localStorage.removeItem(PREVIEW_SESSION_KEY);
}

function getPreviewSession(): PreviewSession | null {
  if (!env.demoModeEnabled) {
    clearPreviewSession();
    return null;
  }
  if (!hasLocalStorage()) {
    return null;
  }

  const raw = localStorage.getItem(PREVIEW_SESSION_KEY);
  if (!raw) {
    return null;
  }

  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!isPreviewSessionPayload(parsed)) {
      localStorage.removeItem(PREVIEW_SESSION_KEY);
      return null;
    }
    // Normaliza role legada (mentor/aluno -> provider/client) para
    // sessoes de preview abertas em versoes anteriores.
    const normalized = normalizeUserType(parsed.role);
    if (!normalized) {
      localStorage.removeItem(PREVIEW_SESSION_KEY);
      return null;
    }
    return { email: parsed.email, role: normalized, token: parsed.token };
  } catch {
    localStorage.removeItem(PREVIEW_SESSION_KEY);
    return null;
  }
}

function setPreviewSession(session: PreviewSession): void {
  if (!env.demoModeEnabled) {
    return;
  }
  if (!hasLocalStorage()) {
    return;
  }

  localStorage.setItem(PREVIEW_SESSION_KEY, JSON.stringify(session));
}

function toPreviewUser(session: PreviewSession): NonNullable<AuthSession["user"]> {
  return {
    id: `preview-${session.role}`,
    email: session.email,
    userType: session.role,
  };
}

function buildPreviewEmail(role: PreviewRole): string {
  return `internal-${role}@preview.local`;
}

function isPreviewRole(value: unknown): value is PreviewRole {
  return (
    value === "admin" || value === "provider" || value === "client"
  );
}

function isPreviewSessionPayload(value: unknown): value is PreviewSession {
  if (!value || typeof value !== "object") {
    return false;
  }
  const candidate = value as Record<string, unknown>;
  return (
    typeof candidate.email === "string" &&
    candidate.email.trim().length > 0 &&
    typeof candidate.token === "string" &&
    candidate.token.trim().length > 0 &&
    isPreviewRole(candidate.role)
  );
}

function normalizeAuthUser(
  payload: { id: number | string; email: string; role?: string | null } | null
): AuthSession["user"] {
  if (!payload) return null;
  const userType = normalizeUserType(payload.role);
  if (!userType) return null;
  return {
    id: payload.id,
    email: payload.email,
    userType,
  };
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const initialPreviewSession = getPreviewSession();
  const [accessToken, setAccessToken] = useState<string | null>(
    initialPreviewSession?.token ?? getAccessToken()
  );
  const [user, setUser] = useState<AuthSession["user"]>(
    initialPreviewSession ? toPreviewUser(initialPreviewSession) : null
  );
  const [loading, setLoading] = useState(
    Boolean(getAccessToken()) && !initialPreviewSession
  );
  const [error, setError] = useState<string | null>(null);
  const [errorCode, setErrorCode] = useState<string | null>(null);
  const [authReady, setAuthReady] = useState(
    Boolean(initialPreviewSession) || !getAccessToken()
  );
  const [sessionRecoveryPending, setSessionRecoveryPending] = useState(false);

  const refreshMe = useCallback(async () => {
    const previewSession = getPreviewSession();
    if (previewSession) {
      setAccessToken(previewSession.token);
      setUser(toPreviewUser(previewSession));
      setError(null);
      setErrorCode(null);
      setSessionRecoveryPending(false);
      setAuthReady(true);
      return;
    }

    if (!getAccessToken()) {
      setAccessToken(null);
      setUser(null);
      setError(null);
      setErrorCode(null);
      setSessionRecoveryPending(false);
      setAuthReady(true);
      return;
    }

    setLoading(true);
    setError(null);
    setErrorCode(null);
    setAuthReady(false);
    try {
      const me = await getMe();
      setAccessToken(getAccessToken());
      setUser(normalizeAuthUser(me));
      setSessionRecoveryPending(false);
    } catch (err) {
      if (err instanceof AppError) {
        if (err.httpStatus === 401) {
          clearAccessToken();
          setAccessToken(null);
          setUser(null);
          setError(null);
          setErrorCode(null);
          setSessionRecoveryPending(false);
          return;
        }
        if (err.isNetworkError) {
          setUser(null);
          setAccessToken(getAccessToken());
          setError(toUserErrorMessage(err, "Falha ao validar sessao."));
          setErrorCode(err.code ?? null);
          setSessionRecoveryPending(true);
          return;
        }
      }

      clearAccessToken();
      setAccessToken(null);
      setUser(null);
      setError(toUserErrorMessage(err, "Falha ao validar sessao."));
      setErrorCode(err instanceof AppError ? err.code ?? null : null);
      setSessionRecoveryPending(false);
    } finally {
      setLoading(false);
      setAuthReady(true);
    }
  }, []);

  const login = useCallback(async (email: string, password: string) => {
    clearPreviewSession();
    setLoading(true);
    setError(null);
    setErrorCode(null);
    setSessionRecoveryPending(false);
    setAuthReady(false);

    try {
      const session = await loginService({ email, password });
      setAccessToken(session.accessToken);
      setUser(normalizeAuthUser(session.user));
      setErrorCode(null);
      setSessionRecoveryPending(false);
      setAuthReady(true);
      return session.user;
    } catch (err) {
      if (err instanceof AppError && err.code === "AUTH_BOOTSTRAP_RETRYABLE") {
        setAccessToken(getAccessToken());
        setUser(null);
        setError(toUserErrorMessage(err, "Falha ao validar seu perfil apos autenticar."));
        setErrorCode(err.code ?? null);
        setSessionRecoveryPending(true);
        setAuthReady(true);
        return null;
      }
      clearAccessToken();
      setAccessToken(null);
      setUser(null);
      setError(toUserErrorMessage(err, "Falha ao autenticar."));
      setErrorCode(err instanceof AppError ? err.code ?? null : null);
      setSessionRecoveryPending(false);
      setAuthReady(true);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  const loginPreview = useCallback((role: PreviewRole) => {
    if (!env.demoModeEnabled) {
      return;
    }

    const previewSession: PreviewSession = {
      email: buildPreviewEmail(role),
      role,
      token: `preview-${role}-token`,
    };

    setPreviewSession(previewSession);
    setAccessToken(previewSession.token);
    setUser(toPreviewUser(previewSession));
    setError(null);
    setErrorCode(null);
    setSessionRecoveryPending(false);
    setAuthReady(true);
  }, []);

  const logout = useCallback(() => {
    logoutService();
    clearPreviewSession();
    clearAccessToken();
    setAccessToken(null);
    setUser(null);
    setError(null);
    setErrorCode(null);
    setSessionRecoveryPending(false);
    setAuthReady(true);
  }, []);

  const retrySessionValidation = useCallback(async () => {
    await refreshMe();
  }, [refreshMe]);

  const clearPendingSession = useCallback(() => {
    clearPreviewSession();
    clearAccessToken();
    setAccessToken(null);
    setUser(null);
    setError(null);
    setErrorCode(null);
    setSessionRecoveryPending(false);
    setAuthReady(true);
  }, []);

  useEffect(() => {
    const unsubscribe = onUnauthorized(() => {
      clearPreviewSession();
      clearAccessToken();
      setAccessToken(null);
      setUser(null);
      setError(null);
      setErrorCode(null);
      setSessionRecoveryPending(false);
      setAuthReady(true);
    });

    return unsubscribe;
  }, []);

  useEffect(() => {
    void refreshMe();
  }, [accessToken, refreshMe]);

  const previewSession = getPreviewSession();
  const isPreviewSession = Boolean(accessToken && previewSession);
  const hasValidatedRealSession = Boolean(
    accessToken && user && isKnownUserRole(user.userType)
  );

  const value = useMemo<AuthContextValue>(
    () => ({
      accessToken,
      user,
      loading,
      error,
      errorCode,
      authReady,
      isAuthenticated: authReady && (isPreviewSession || hasValidatedRealSession),
      isPreviewSession,
      canUsePreviewLogin: env.demoModeEnabled,
      sessionRecoveryPending,
      login,
      loginPreview,
      logout,
      refreshMe,
      retrySessionValidation,
      clearPendingSession,
    }),
    [
      accessToken,
      user,
      loading,
      error,
      errorCode,
      authReady,
      isPreviewSession,
      hasValidatedRealSession,
      sessionRecoveryPending,
      login,
      loginPreview,
      logout,
      refreshMe,
      retrySessionValidation,
      clearPendingSession,
    ]
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth precisa de AuthProvider no topo da aplicacao.");
  }
  return context;
}
