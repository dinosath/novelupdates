#!/bin/bash

# Build a JSON object from all files in entities/
entities_json=$(jq -n 'reduce inputs as $item ({}; . + {($item|input_filename|split("/")|last|rtrimstr(".json")): $item})' entities/*.json)

# Merge into grpc-crud-answers.json
jq --argjson entities "$entities_json" '.entities = $entities' answers.json > tmp.json && mv tmp.json answers.json

