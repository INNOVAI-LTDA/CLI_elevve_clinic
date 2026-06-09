import { Link, Outlet, useLocation } from "react-router-dom";
import { useAuth } from "../providers/AuthProvider";
import { env } from "../../shared/config/env";

/**
 * F2 refactor: o switch de role usa a nomenclatura unificada
 * (Admin | Provider | Client) em vez do antigo (Admin | Mentor | Aluno).
 * Removida a variavel morta `canUsePublishedMentorWorkspace`.
 */

function getRoleLabel(role: string | undefined): string {
  switch (role) {
    case "admin":
      return "Admin";
    case "provider":
      return "Provider";
    case "client":
      return "Client";
    default:
      return "Visitante";
  }
}

export function AppLayout() {
  const location = useLocation();
  const { isAuthenticated, isPreviewSession, user, logout } = useAuth();
  const role = user?.userType;

  if (location.pathname === "/login") {
    return (
      <main>
        <Outlet />
      </main>
    );
  }

  return (
    <div className="app-shell">
      <header className="app-topbar">
        <nav className="app-topbar__inner">
          <Link className="app-brand" to="/login">
            <img src={env.brandingIconUrl} alt="" aria-hidden="true" />
            <span>
              <strong>{env.clientName}</strong>
              <small>{env.appName}</small>
            </span>
          </Link>
          <span className="app-session-indicator">
            {isAuthenticated
              ? `${isPreviewSession ? "Sessão interna" : "Sessão"}: ${getRoleLabel(role)}`
              : "Sessão: anônima"}
          </span>
          {isAuthenticated && (
            <button
              className="app-logout-button"
              type="button"
              onClick={logout}
            >
              Sair
            </button>
          )}
        </nav>
      </header>
      <main className="app-main">
        <Outlet />
      </main>
    </div>
  );
}
