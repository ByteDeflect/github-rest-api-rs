#!/bin/sh

export _JAVA_OPTIONS=-DmaxYamlCodePoints=99999999
openapi-generator generate -i "https://github.com/github/rest-api-description/raw/refs/heads/main/descriptions/api.github.com/api.github.com.2022-11-28.yaml" -g rust -o . --api-package "github-rest-api"
