import { Link } from "react-router-dom";
import { useAuth } from "../app/providers/AuthProvider";
import { getDefaultRouteForRole, getRoleHomeLabel, isKnownUserRole } from "../shared/auth/roleRouting";

export function AccessDeniedPage() {
  const { user } = useAuth();
  const userType = user?.userType;
  const destination =
    userType && isKnownUserRole(userType)
      ? getDefaultRouteForRole(userType)
      : "/app";
  const destinationLabel =
    userType && isKnownUserRole(userType)
      ? getRoleHomeLabel(userType)
      : "voltar ao inicio";

  return (
    <section className="page">
      <div className="card">
        <p className="eyebrow">Erro 403</p>
        <h1>Acesso negado</h1>
        <p>Seu perfil autenticado nao tem permissao para acessar esta area desta entrega.</p>
        <Link className="button-link" to={destination}>
          {destinationLabel}
        </Link>
      </div>
    </section>
  );
}
