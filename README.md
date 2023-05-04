# README

This project is a demonstration of the organization of the solution when writing a project in a microservice style.

To organize a set of commands into a project, use [Just](https://github.com/casey/just).

---

For demonstration use:

- SSO: [keycloak](https://keycloak.org)
  
- Web-client: react + typescript + [react-oidc-context](https://github.com/authts/react-oidc-context) (there is also a project version with [@axa-fr/react-oidc](https://github.com/erritis/actix-react-oidc-demo/tree/axa-fr_react-oidc))
  
- Microservice: [actix-web](https://github.com/actix/actix-web) + [aliri](https://github.com/neoeinstein/aliri) (there is also a project version with [actix-4-jwt-auth](https://github.com/erritis/actix-react-oidc-demo/tree/actix-4-jwt-auth))

---

### Users in keycloak:

| login        | password                         |
|--------------|:--------------------------------:|
| admin        | admin                            |
| react-tester | ^J2WnUY#T3XSi4@AH4NEySqUAhfhid%o |

---

## Deployment

There are two deployment methods:

- [Deployment in docker compose](docs/docker-deployment.md)

- [Deployment to minikube](docs/minikube-deployment.md)