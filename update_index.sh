#!/bin/bash
set -e

# Compile the auto_index.rs script if it's newer than the compiled binary
if [ ! -f "scripts/auto_index" ] || [ "scripts/auto_index.rs" -nt "scripts/auto_index" ]; then
    echo "Compiling auto_index.rs..."
    rustc scripts/auto_index.rs -o scripts/auto_index
fi

# Run the autogenerator
echo "Running auto-indexer..."
./scripts/auto_index

echo "Done!"
