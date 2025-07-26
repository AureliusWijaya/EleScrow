#!/bin/bash

set -e

echo "Generating Candid files for elescrow_backend..."

mkdir -p candid

echo "📝 Running cargo test to generate candid file..."
cargo test save_candid

if [ -f "candid/elescrow_backend.did" ]; then
    echo "✅ Candid file generated successfully: candid/elescrow_backend.did"
    echo "📄 Contents:"
    echo "----------------------------------------"
    cat candid/elescrow_backend.did
    echo "----------------------------------------"
else
    echo "❌ Failed to generate candid file"
    exit 1
fi

echo "🎉 Candid generation complete!"