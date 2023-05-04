**The first deployment option** is to demonstrate how to set up [OpenID Connect](https://openid.net/connect) on both the server and the client using [SSO](https://auth0.com/intro-to-iam/what-is-single-sign-on-sso). This is a simplified deployment option that should not be used in real projects.

For demonstration use:

- SSO: [keycloak](https://keycloak.org)
  
- Web-client: react + typescript + [react-oidc-context](https://github.com/authts/react-oidc-context) (there is also a project version with [@axa-fr/react-oidc](https://github.com/erritis/actix-react-oidc-demo/tree/axa-fr_react-oidc))
  
- Microservice: [actix-web](https://github.com/actix/actix-web) + [aliri](https://github.com/neoeinstein/aliri) (there is also a project version with [actix-4-jwt-auth](https://github.com/erritis/actix-react-oidc-demo/tree/actix-4-jwt-auth))

---

## The first deployment option

---

#### To run you need to run the command:

> docker compose up

---
### Links with ports in localhost:

- KeycloakDB (Postgres): http://localhost:52632

- Keycloak: http://localhost:52633

- Microservice on Actix Web: http://localhost:52634

- Web-client: http://localhost:52635

---