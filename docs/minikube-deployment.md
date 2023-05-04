**Deployment to Minikube** is the same as the first option, but in addition it also serves to demonstrate how CI/CD can be organized.

For demonstration use:

- Orchestrator: Kubernetes (used [minikube](https://minikube.sigs.k8s.io/docs/start) for this example)

- Container repository (this example uses [private repository](https://werf.io/documentation/v1.2/#check-the-result) deployed in [minikube](https://minikube.sigs.k8s.io/docs/start/))

- [Kompose](https://kompose.io/installation) - to make it easier to write configuration files in the style of docker-compose files. With subsequent conversion to werf templates.

- Tool for building and deploying: [Werf](https://werf.io/installation.html) - allows you to build a container only after a commit in the git with the subsequent deployment of containers to kubernetes according to the described templates.

---

## Deployment to Minikube

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
> In case of problems with the deployment (and they will be), check out the [werf deployment section](werf-deployment.md)

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
> In case of problems with the deployment (and they will be), check out the [werf deployment section](werf-deployment.md)

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
  
### Debugging in kubernetes

An article about how debugging works in Kubernetes: [Use Bridge to Kubernetes](https://learn.microsoft.com/en-us/visualstudio/bridge/bridge-to-kubernetes-vs-code?view=vs-2019)

> **Warning**
> 
> In case of problems with configuring the plugin (and they most likely will), read the section: [Problems when configuring a kubernetes plugin](bridge-to-kubernetes.md)

The project already contains ready-made configurations for replacing services in isolated mode

In order for debugging to work in this mode, you need to specify the **kubernetes-route-as** header in each request when accessing kubernetes addresses.

To simplify this task, you can use a browser plugin that automatically adds headers. I used the open source solution: [SimpleModifyHeaders](https://github.com/didierfred/SimpleModifyHeaders)

Here is an example of my settings:

>   **Url Patterns\***: http://actix-react-oidc.test/\*;http://\*.actix-react-oidc.test/\*
>
> >     Header Field Name: kubernetes-route-as
> >     Header Field Value: actix-react-oidc-demo-2edd

In case you do not need isolated mode, remove or comment out the **isolateAs** field in the **/.vscode/tasts.json** file of the project for which you want to disable this mode:

>       {
>       	"version": "2.0.0",
>       	"tasks": [
>       		{
>       			"label": "bridge-to-kubernetes.resource",
>                   ...
>       			# "isolateAs": "actix-react-oidc-demo-2edd"
>       		}
>       	]
>       }

When debugging a web-client, you can still access it by its domain name.