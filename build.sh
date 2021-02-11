#!/bin/bash

source credentials.env || true

docker build --build-arg artifact_auth="$ARTIFACT_AUTH" -t gcr.io/github-artifact/github-artifact .
