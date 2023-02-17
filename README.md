# README

This project is a demonstration of the organization of the solution when writing a project in a microservice style.

To organize a set of commands into a project, use [Just](https://github.com/casey/just).

---

**The first deployment option** is to demonstrate how to set up [OpenID Connect](https://openid.net/connect) on both the server and the client using [SSO](https://auth0.com/intro-to-iam/what-is-single-sign-on-sso). This is a simplified deployment option that should not be used in real projects.

For demonstration use:

- SSO: [keycloak](https://keycloak.org)
  
- Web-client: react + typescript + [react-oidc-context](https://github.com/authts/react-oidc-context) (there is also a project version with [@axa-fr/react-oidc](https://github.com/erritis/actix-react-oidc-demo/tree/axa-fr_react-oidc))
  
- Microservice: [actix-web](https://github.com/actix/actix-web) + [aliri](https://github.com/neoeinstein/aliri) (there is also a project version with [actix-4-jwt-auth](https://github.com/erritis/actix-react-oidc-demo/tree/actix-4-jwt-auth))

---

**The second deployment option** is the same as the first option, but in addition it also serves to demonstrate how CI/CD can be organized.

For demonstration use:

- Orchestrator: Kubernetes (used [minikube](https://minikube.sigs.k8s.io/docs/start) for this example)

- Container repository (this example uses [private repository](https://werf.io/documentation/v1.2/#check-the-result) deployed in [minikube](https://minikube.sigs.k8s.io/docs/start/))

- [Kompose](https://kompose.io/installation) - to make it easier to write configuration files in the style of docker-compose files. With subsequent conversion to werf templates.

- Tool for building and deploying: [Werf](https://werf.io/installation.html) - allows you to build a container only after a commit in the git with the subsequent deployment of containers to kubernetes according to the described templates.

---
### Users in keycloak:

| login        | password                         |
|--------------|:--------------------------------:|
| admin        | PdZoV9NpYuq#zdsYXfHo             |
| react-tester | ^J2WnUY#T3XSi4@AH4NEySqUAhfhid%o |

---

## The first deployment option

---

> **Warning**
> 
> This example is tested only on Linux, if you use a hypervisor, you may need to make some changes.

---

#### To run you need to run the command:

> docker compose up

---

> **Warning**
> ##### The ip address used (default: 172.26.0.0/24) specified in docker compose may already be taken, in this case, look at what is currently available for you and execute (format: x.x.0.0/24):
> 
>> just compose-set-network 172.27.0.0/24

---
### Links with ports in localhost:

- KeycloakDB (Postgres): http://localhost:52632

- Keycloak: http://localhost:52633

- Microservice on Actix Web: http://localhost:52634

- Web-client: http://localhost:52635

---
## The second deployment option

---
### Installation and setup

Install:

- [minikube](https://minikube.sigs.k8s.io/docs/start/)

- [helm](https://helm.sh/docs/intro/install/)

- [kompose](https://kompose.io/installation/)

- [werf](https://werf.io/installation.html)

Set up ingress in minikube like in [this article](https://minikube.sigs.k8s.io/docs/handbook/addons/ingress-dns/).

> **Warning**
> 
> Use only the zones specified in the article (For example, .test or .example)

Set up a private repository in minikube for werf (but any will do), as in [this article](https://werf.io/documentation/v1.2/#check-the-result).

> **Warning**
> 
> In minikube, the *--insecure-registry* parameter must be specified on first launch, otherwise minikube will not be able to fetch containers from the non-SSL repository.
> 
> To rollback changes without losing ingress settings, run:
> 
> > minikube delete
>
> > minikube start --driver=docker --insecure-registry registry.test:80
> 
> Please note that addons will still be lost and will need to be reinstalled.

---

### Setting up deployment configs

To change deployment configurations, edit **docker-compose.werf.yml**.

Run the command:

> just werf-convert

After this command, in the **./.helm/teemplates/** folder, the deployment templates should be replaced with the changes made.

Run:

> git add .
>
> git commit

---

### Build and Deploy

Set the host and port of the container repository to use:

> just werf-set-repo registry.test:80

Run the command:

> just werf-up

---

> **Warning**
> 
> If it's complaining about HTTPS (SSL) when trying to push a container to a repository, it's most likely that the *$DOCKER_OPTS* environment variable pointing to **/etc/docker/daemon.json** is not being used when starting the docker service.
>
> Add to **/etc/default/docker** file:
>
> >     DOCKER_OPTS="--config-file=/etc/docker/daemon.json"
> 
> Change the docker service description:
>
> >     EnvironmentFile=-/etc/default/docker
> >     ExecStart=/usr/bin/dockerd -H fd:// $DOCKER_OPTS 
>
> Run:
>
> > sudo systemctl daemon-reload
> >
> > sudo systemctl restart docker.service
>
> Check:
>
> > docker info
>
> If that doesn't help add to **~/.profile**:
>
> >     export WERF_INSECURE_REGISTRY=1
> >     export DOCKER_OPTS="--config-file=/etc/docker/daemon.json"
>
> And in **~/.bash_profile**:
> 
> >      if [[ -f ~/.profile ]] && . ~/.profile
>
> If that doesn't work add to **/etc/environment**:
> >     DOCKER_OPTS="--config-file=/etc/docker/daemon.json"

To remove containers from kubernetes run:

> just werf-down

For a complete cleaning:

> just werf-cleanup

---

### Links indicating ports in minikube:

- KeycloakDB (Postgres): http://keycloakdb.actix-react-oidc.test:5432

- Keycloak: http://keycloak.actix-react-oidc.test

- Microservice on Actix Web: http://backend.actix-react-oidc.test

- Web-client: http://actix-react-oidc.test
  
  