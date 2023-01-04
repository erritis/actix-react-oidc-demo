import { useAuth } from "react-oidc-context";
import { FC } from "react";

const PrivateRoute: FC<{ children: JSX.Element }> = ({ children }) => {
  const auth = useAuth();

  const isLoggedIn = auth && auth.user;

  return isLoggedIn ? children : null;
};

export default PrivateRoute;
