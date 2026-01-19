#!/usr/bin/env bash

set -e

PLUGIN_TYPE="$1"        # transform | hpc
NAME="$2"               # transform_name
SUBFOLDER="$3"          # optional_sub_folder

if [[ -z "$PLUGIN_TYPE" || -z "$NAME" ]]; then
  echo "Usage: $0 <transform|hpc> <name> [subfolder]"
  exit 1
fi

DESCRIPTOR_PATH="../../packages/${PLUGIN_TYPE}s/${NAME}/src/rhp.json"
CODE_PATH="/Users/veronica/projects/rhexis/target/debug/lib${NAME}.dylib"

if [[ -n "$SUBFOLDER" ]]; then
  OUTPUT_PATH="../../rhexis-startup/src/${SUBFOLDER}/${NAME}-macos_arm64-v1.rhp"
else
  OUTPUT_PATH="../../rhexis-startup/src/${NAME}-macos_arm64-v1.rhp"
fi

echo "📦 Packing RHP:"
echo "  Type:       $PLUGIN_TYPE"
echo "  Name:       $NAME"
echo "  Descriptor: $DESCRIPTOR_PATH"
echo "  Code:       $CODE_PATH"
echo "  Output:     $OUTPUT_PATH"
echo

./rhp pack\
  --plugin-type "$PLUGIN_TYPE" \
  --descriptor-path "$DESCRIPTOR_PATH" \
  --code-path "$CODE_PATH" \
  --output-path "$OUTPUT_PATH"

echo
echo "✨ RHP forged: $OUTPUT_PATH"
