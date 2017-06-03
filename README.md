
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
