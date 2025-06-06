#!/bin/bash

# Stable Port Forward Maintenance Script
# Keeps Alice (9944) and Bob (9945) accessible even during pod restarts

NAMESPACE="fennel"
ALICE_PORT="9944"
BOB_PORT="9945"

echo "🔌 Starting persistent port forwarding..."

# Function to start Alice port forward
start_alice() {
    echo "🔗 Starting Alice port forward (9944)..."
    nohup kubectl port-forward -n $NAMESPACE svc/fennel-solochain-node $ALICE_PORT:9944 > /tmp/alice-port-forward.log 2>&1 &
    ALICE_PID=$!
    echo "✅ Alice port forward PID: $ALICE_PID"
}

# Function to start Bob port forward  
start_bob() {
    echo "🔗 Starting Bob port forward (9945)..."
    nohup kubectl port-forward -n $NAMESPACE fennel-solochain-node-1 $BOB_PORT:9944 > /tmp/bob-port-forward.log 2>&1 &
    BOB_PID=$!
    echo "✅ Bob port forward PID: $BOB_PID"
}

# Function to check if port forward is alive
check_port() {
    local port=$1
    curl -s --max-time 2 http://localhost:$port/health > /dev/null 2>&1
    return $?
}

# Kill any existing port forwards
echo "🧹 Cleaning up existing port forwards..."
pkill -f "kubectl port-forward" 2>/dev/null || true
sleep 2

# Start both port forwards
start_alice
start_bob

echo ""
echo "📋 Port Forward Status:"
echo "   Alice (fennel-solochain-node):   http://localhost:$ALICE_PORT"
echo "   Bob   (fennel-solochain-node-1): http://localhost:$BOB_PORT"
echo ""
echo "📝 Logs:"
echo "   Alice: tail -f /tmp/alice-port-forward.log"
echo "   Bob:   tail -f /tmp/bob-port-forward.log"
echo ""
echo "🔄 To restart if needed: $0"
echo "🛑 To stop: pkill -f 'kubectl port-forward'"

# Keep script running to show status
while true; do
    sleep 30
    echo "$(date): Port forwards running - Alice:$ALICE_PORT Bob:$BOB_PORT"
done 