old_repo := "registry.test:80"

werf-set-repo repo:
  old_repo={{old_repo}} \
  && repo={{repo}} \
  && sed -i "s/$(echo $old_repo | sed -e 's/[\/&]/\\&/g')/$(echo $repo | sed -e 's/[\/&]/\\&/g')/g" .justfile;
werf-convert:
  kompose convert -f docker-compose.werf.yml -o ./.helm/templates;
  rm ./.helm/templates/keycloakdb-persistentvolumeclaim.yaml;
  find ./.helm/templates -type f -exec sed -i "s/'{{{{ \(.*\) }}'/{{{{ \1 }}/g" {} +;
werf-up:
  werf converge --repo {{old_repo}}/actix-react-oidc-demo
werf-down:
  werf dismiss --repo {{old_repo}}/actix-react-oidc-demo
werf-cleanup:
  werf cleanup --repo {{old_repo}}/actix-react-oidc-demo