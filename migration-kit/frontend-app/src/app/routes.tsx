import { Navigate, Outlet, createBrowserRouter, type RouteObject } from "react-router-dom";
import { AppLayout } from "./layout/AppLayout";
import { LoginPage } from "../pages/LoginPage";
import { NotFoundPage } from "../pages/NotFoundPage";
import { AccessDeniedPage } from "../pages/AccessDeniedPage";

import { BiRadarPage } from "../features/bi-radar/pages/BiRadarPage";

import { useAuth } from "./providers/AuthProvider";
import { getDefaultRouteForRole, isKnownUserRole } from "../shared/auth/roleRouting";
import { env } from "../shared/config/env";

/**
 * F2 refactor: rotas do Acelerador Medico (centro-comando, radar
 * antigo, matriz-renovacao, aluno, admin, measurements, etc.)
 * foram dropadas. O BI Elevve e' o unico produto do frontend.
 *
 * F3 (proxima fase) vai deletar os arquivos de
 * `features/{radar,matrix,command-center,mentor,student,admin,...}/`
 * que ficaram sem uso.
 */

function AuthLoadingFallback() {
  return (
    <main className="page">
      <p>Validando sessao...</p>
    </main>
  );
}

function RequireAuth() {
  const { authReady, isAuthenticated } = useAuth();

  if (!authReady) {
    return <AuthLoadingFallback />;
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <Outlet />;
}

function RoleHomeRedirect() {
  const { authReady, isAuthenticated, user } = useAuth();

  if (!authReady) {
    return <AuthLoadingFallback />;
  }

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  if (!user || !isKnownUserRole(user.userType)) {
    return <Navigate to="/login" replace />;
  }

  return <Navigate to={getDefaultRouteForRole(user.userType)} replace />;
}

export const appRoutes: RouteObject[] = [
  {
    path: "/",
    element: <AppLayout />,
    children: [
      { index: true, element: <Navigate to="/login" replace /> },
      { path: "login", element: <LoginPage /> },
      { path: "dashboard", element: <Navigate to="/app" replace /> },
      {
        path: "app",
        element: <RequireAuth />,
        children: [
          { index: true, element: <RoleHomeRedirect /> },
          { path: "acesso-negado", element: <AccessDeniedPage /> },
          // F2: rotas do BI Elevve (placeholder; F4+ adiciona
          // overview, historico, relatorio)
          { path: "bi/radar", element: <BiRadarPage /> },
        ],
      },
      { path: "*", element: <NotFoundPage /> },
    ],
  },
];

export const appRouter = createBrowserRouter(appRoutes, {
  basename: env.routerBasePath,
});
