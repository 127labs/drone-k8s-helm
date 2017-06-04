
## YAML File

```yaml
pipeline:
  publish:
    image: 127labs/yakp
    chart: "./charts/yakp"
    master: "https://<master-kubernetes-ip>"
    release: "<yakp-prod>"
    tls: true
    token: 0xdeadbeef
    values:
      - HOSTNAME=myapp.dev
      - PORT=4000
    secrets: [plugin_set_kubernetes_token, plugin_set_whatever]
```

## Testing

```
docker run --rm 127labs/yakp \
  -e PLUGIN_CHART=<string> \
  -e PLUGIN_MASTER=<string> \
  -e PLUGIN_RELEASE=<string> \
  -e PLUGIN_SKIP_TLS=<bool> \
  -e PLUGIN_TOKEN=<string> \
  -e PLUGIN_VALUES=<comma-separated-key-value-pair> \
```
