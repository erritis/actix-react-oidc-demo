old_ip_set := "172.26.0.0/24"
old_repo := "registry.test:80"

compose-set-network ip_set:
  old_ip_set={{old_ip_set}} \
  && old_ip_set="${old_ip_set:0:-4}" \
  && ip_set={{ip_set}} \
  && ip_set="${ip_set:0:-4}" \
  && sed -i "s/$(echo $old_ip_set | sed -e 's/[\/&]/\\&/g')/$(echo $ip_set | sed -e 's/[\/&]/\\&/g')/g" .justfile \
  && sed -i "s/$(echo $old_ip_set | sed -e 's/[\/&]/\\&/g')/$(echo $ip_set | sed -e 's/[\/&]/\\&/g')/g" docker-compose.yml \
  && sed -i "s/$(echo $old_ip_set | sed -e 's/[\/&]/\\&/g')/$(echo $ip_set | sed -e 's/[\/&]/\\&/g')/g" src/react/public/OidcTrustedDomains.js;
  
werf-set-repo repo:
  old_repo={{old_repo}} \
  && repo={{repo}} \
  && sed -i "s/$(echo $old_repo | sed -e 's/[\/&]/\\&/g')/$(echo $repo | sed -e 's/[\/&]/\\&/g')/g" .justfile;
werf-convert:
  kompose convert -f docker-compose.werf.yml -o ./.helm/templates;
  rm ./.helm/templates/keycloakdb-persistentvolumeclaim.yaml;
  find ./.helm/templates -type f -exec sed -i "s/'{{{{ \(.*\) }}'/{{{{ \1 }}/g" {} +;
werf-up:
  werf converge --repo {{old_repo}}/example-actix-react-auth
werf-down:
  werf dismiss --repo {{old_repo}}/example-actix-react-auth
werf-cleanup:
  werf cleanup --repo {{old_repo}}/example-actix-react-auth