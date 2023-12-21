#!/bin/sh

# Define the path to the backend binary
path="$HOME/.local/bin/cdf_backend"

# Build the release binary
cargo build --release
# $? checks the exit status of the last command
if [ $? -ne 0 ]; then
  echo "Build failed, exiting script."
  exit 1
fi

# Remove the existing binary, if it exists
rm -f "$path"

# Copy the new binary to the target path
cp "./target/release/cdf" "$path"

# Verify the copy operation was successful
if [ $? -eq 0 ]; then
  echo "cdf binary successfully updated."
else
  echo "Failed to copy cdf binary."
fi
