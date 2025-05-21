#!/bin/bash
set -e

echo "Testing the Fennel Solonet Docker container..."

# Check if image exists
if ! docker image inspect fennel-solonet-test &> /dev/null; then
  echo "Error: The fennel-solonet-test image doesn't exist. Build it first with:"
  echo "docker build -t fennel-solonet-test ."
  exit 1
fi

echo "Running version check..."
# Run the container with --version to verify it works
docker run --rm fennel-solonet-test /app/fennel-node --version

echo "Starting container in detached mode..."
# Run the container in detached mode to verify it starts properly
CONTAINER_ID=$(docker run -d --rm -p 9944:9944 fennel-solonet-test /app/fennel-node --dev --ws-external)

echo "Container started with ID: $CONTAINER_ID"
echo "Waiting 10 seconds for node to initialize..."
sleep 10

echo "Checking container logs..."
docker logs $CONTAINER_ID

echo "Stopping container..."
docker stop $CONTAINER_ID

echo "Test completed successfully!" 