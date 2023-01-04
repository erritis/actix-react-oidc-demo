import { OidcConfiguration } from "@axa-fr/react-oidc";

export const oidcConfig = {
  client_id: process.env.REACT_APP_KEYCLOAK_CLIENT_ID,
  redirect_uri: window.location.origin + '/authentication/callback',
  silent_redirect_uri: window.location.origin + '/authentication/silent-callback',
  scope: 'openid profile email offline_access',
  authority: process.env.REACT_APP_KEYCLOAK_URL,
  service_worker_relative_url:'/OidcServiceWorker.js',
  service_worker_only:true,
} as OidcConfiguration;