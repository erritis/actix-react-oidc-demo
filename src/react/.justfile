environment := "development"

react-local-up:
  REACT_APP_ENVIRONMENT_NAME={{environment}} \
  NODE_ENV={{environment}} \
  PORT=3000 \
  REACT_APP_KEYCLOAK_CLIENT_ID=react \
  REACT_APP_KEYCLOAK_URL=http://localhost:52633/realms/actix-react-demo \
  REACT_APP_BACKEND_URL=http://localhost:52634 \
  yarn start

react-kube-up:
  REACT_APP_ENVIRONMENT_NAME={{environment}} \
  NODE_ENV={{environment}} \
  PORT=3000 \
  REACT_APP_KEYCLOAK_CLIENT_ID=web \
  REACT_APP_KEYCLOAK_URL=http://keycloak.actix-react-oidc.test/realms/actix-react-demo \
  REACT_APP_BACKEND_URL=http://backend.actix-react-oidc.test \
  yarn start