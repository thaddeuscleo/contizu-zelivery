#! /bin/bash
helm install istio-base istio/base -n istio-system --values "./istio/values.yaml" --create-namespace
