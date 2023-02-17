# README

This project is a demonstration of the organization of the solution when writing a project in a microservice style.

To organize a set of commands into a project, use [Just](https://github.com/casey/just).

---

For demonstration use:

- SSO: [keycloak](https://keycloak.org)
  
- Web-client: react + typescript + [@axa-fr/react-oidc](https://github.com/AxaGuilDEv/react-oidc) (there is also a project version with [react-oidc-context](https://github.com/erritis/actix-react-oidc-demo/tree/master))
  
- Microservice: [actix-web](https://github.com/actix/actix-web) + [aliri](https://github.com/neoeinstein/aliri) (there is also a project version with [actix-4-jwt-auth](https://github.com/erritis/actix-react-oidc-demo/tree/actix-4-jwt-auth))

> **Warning**
> 
> At the moment, ServerWorker from the [@axa-fr/react-oidc] library behaves incorrectly in kubernetes.

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

- [Deployment to Minikube](docs/minikube-deployment.md)