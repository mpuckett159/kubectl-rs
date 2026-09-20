# kubectl-rs
kubectl rewritten in rust, for users not for CI systems

# Examples

Gets all pods in the kube-system namespace.

```bash
kubectl-rs get "" v1 pod -n kube-system
```

Gets a pod by name in the kube-system namespace.

```bash
kubectl-rs get "" v1 pod pod-name -n kube-system
```

Create a new pod using a yaml file

```bash
kubectl-rs create pod.yaml
```

Patch a new pod using a yaml file with server-side apply. kubectl-rs does not support any patch types besides server-side apply

```bash
kubectl-rs patch pod.yaml
```

Delete a new pod using a yaml file

```bash
kubectl-rs delete pod.yaml
```

View your current config file

```bash
kubectl-rs config view
```

# Multi-context gets

You need to create a file like so to do multi-context gets

```yaml
apiVersion: v1alpha1
kind: Groups
contextGroups:
  dev:
    - kind-kind
    - kind-kind2
```

The default file path is `~/.kube/groups.yaml`.

Get from the context group dev, which contains kind-kind and kind-kind2.

```bash
kubectl-rs get "" v1 pod -n kube-system --context-group dev
```

Specify a custom file location

```bash
kubectl-rs get "" v1 pod -n kube-system --context-group dev --context-file ./groups.yaml
```

# This is basically a science experient at the moment do not expect anything to work well. This has not been tested against anything besides local kind based clusters, EKS/GKE/AKS likely do not work.
