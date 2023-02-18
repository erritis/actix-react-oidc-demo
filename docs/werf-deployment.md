# Fixing problems with deployment via Werf

---

## If minikube fails to get containers from the repository.

In minikube, the *--insecure-registry* parameter must be specified on first launch, otherwise minikube will not be able to fetch containers from the non-SSL repository.

To rollback changes without losing ingress settings, run:

> minikube delete

> minikube start --driver=docker --insecure-registry registry.test:80

Please note that addons will still be lost and will need to be reinstalled.

---

## If complains about HTTPS (SSL) when trying to push a container to a repository.

It's most likely that the *$DOCKER_OPTS* environment variable pointing to **/etc/docker/daemon.json** is not being used when starting the docker service.

Add to **/etc/default/docker** file:

>     DOCKER_OPTS="--config-file=/etc/docker/daemon.json"

Change the docker service description:

>     EnvironmentFile=-/etc/default/docker
>     ExecStart=/usr/bin/dockerd -H fd:// $DOCKER_OPTS 

Run:

> sudo systemctl daemon-reload
>
> sudo systemctl restart docker.service

Check:

> docker info

If that doesn't help add to **~/.profile**:

>     export WERF_INSECURE_REGISTRY=1
>     export DOCKER_OPTS="--config-file=/etc/docker/daemon.json"

And in **~/.bash_profile**:

>      if [[ -f ~/.profile ]] && . ~/.profile

If that doesn't work add to **/etc/environment**:
>     DOCKER_OPTS="--config-file=/etc/docker/daemon.json"
  
  