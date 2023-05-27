#!/bin/bash -ex

# TODO: cross-platform?
# FIXME: ignore CRLF differences

# Fail if changed. See https://stackoverflow.com/a/9393642
[[ -z $(git status -s) ]]
