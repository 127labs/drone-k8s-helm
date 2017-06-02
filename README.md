
## YAML File

```yaml
pipeline:
  publish:
    image: 127labs/yakp
    values:
      - HOSTNAME=myapp.dev
      - PORT=4000
    secrets: [plugin_set_kubernetes_token, plugin_set_whatever]
```
