## Example Deployment

```yaml
pipeline:
  ship:
    image: 127labs/yakp
    chart: stable/ghost
    master: https://127.0.0.1:8001
    release: production
    skip_tls: true
    values:
      - image=bitnami/ghost:0.11.9-alpine
    secrets: [plugin_token]
```

Which is equivalent to

```shell
docker run 127labs/yakp \
  -e PLUGIN_CHART=stable/ghost \
  -e PLUGIN_MASTER=https://127.0.0.1:8001 \
  -e PLUGIN_RELEASE=production \
  -e PLUGIN_SKIP_TLS=true \
  -e PLUGIN_VALUES=image=bitnami/ghost:0.11.0-alpine
  -e PLUGIN_TOKEN=<secret>
```
