#!/bin/bash

RUST_LOG=debug cat answers.json | baker ../../springrs-template .. --answers=- --non-interactive --force --verbose --skip-confirms all
