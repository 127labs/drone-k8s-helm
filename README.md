## Example Deployment

```yaml
pipeline:
  ship:
    image: 127labs/yakp
    chart: stable/dokuwiki
    master: https://127.0.0.1:8001
    token: super-long-token
    release: wiki
    skip_tls: true
    clean_before_release: true
    values:
      dokuwikiEmail: $${DOKUWIKI_EMAIL}
      dokuwikiPassword: $${DOKUWIKI_PASSWORD}
    secrets: [dokuwiki_email, dokuwiki_password]
```

Which is equivalent to

```shell
docker run 127labs/yakp \
  -e PLUGIN_CHART=stable/dokuwiki \
  -e PLUGIN_MASTER=https://127.0.0.1:8001 \
  -e PLUGIN_TOKEN=super-long-token \
  -e PLUGIN_RELEASE=wiki \
  -e PLUGIN_SKIP_TLS=true \
  -e PLUGIN_CLEAN_BEFORE_RELEASE=true \
  -e PLUGIN_VALUES='{"dokuwikiEmail":"${DOKUWIKI_EMAIL}","dokuwikiPassword":"${DOKUWIKI_PASSWORD}"}' \
  -e DOKUWIKI_EMAIL=imran@127labs.com \
  -e DOKUWIKI_PASSWORD=password
```
