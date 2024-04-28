#!/bin/bash
./copy_entities.bash
RUST_LOG=debug cat answers.json | baker ../../axum-template .. --answers=- --non-interactive --force --skip-confirms all
RUST_LOG=debug cat answers.json | baker ../../shadcn-admin-kit-template ../admin --answers=- --non-interactive --force --skip-confirms all
