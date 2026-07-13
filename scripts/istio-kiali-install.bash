#! /bin/bash
helm install kiali-operator kiali/kiali-operator --namespace kiali-operator --create-namespace --values "./istio/kiali-values.yaml"

# helm upgrade kiali-operator kiali/kiali-operator --namespace kiali-operator --values "./istio/kiali-values.yaml"
