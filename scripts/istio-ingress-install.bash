#! /bin/bash

helm install istio-ingressgateway istio/gateway -n istio-ingress --values "./istio/gateway-values.yaml" --create-namespace

# helm upgrade istio-ingressgateway istio/gateway -n istio-ingress --values "./istio/gateway-values.yaml"
