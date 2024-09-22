#!/bin/sh

export _JAVA_OPTIONS=-DmaxYamlCodePoints=99999999
openapi-generator generate -i api.github.com.yaml -g rust -o . --api-package "github-rest-api"