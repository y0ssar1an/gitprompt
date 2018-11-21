#!/usr/bin/env zsh

CGO_ENABLED=0 go install -ldflags '-s -w'
