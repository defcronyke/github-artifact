#!/bin/bash

docker build --build-arg artifact_auth="$ARTIFACT_AUTH" -t gcr.io/github-artifact/github-artifact .
