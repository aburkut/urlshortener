#!/bin/bash
# Run tests fully in Docker (no local dependencies needed)

set -e

echo "🐳 Running tests in Docker..."
echo ""

# Build and run tests
docker-compose --profile test up --build --abort-on-container-exit test

# Capture exit code
EXIT_CODE=$?

# Cleanup
echo ""
echo "🧹 Cleaning up..."
docker-compose --profile test down

if [ $EXIT_CODE -eq 0 ]; then
    echo ""
    echo "✅ All tests passed!"
else
    echo ""
    echo "❌ Tests failed with exit code $EXIT_CODE"
fi

exit $EXIT_CODE
