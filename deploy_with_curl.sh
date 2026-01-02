#!/bin/bash

# Configuration
NODE_URL="https://node.testnet.cspr.cloud/rpc"
ACCESS_TOKEN="<access-token>"
CHAIN_NAME="casper-test"
SECRET_KEY_PATH="<secret-path>"
WASM_PATH="<wasm-path>"

echo "🚀 Deploying CasperShield Contract to Testnet..."
echo "📡 Node: $NODE_URL"
echo "🔗 Chain: $CHAIN_NAME"
echo "📄 WASM: $WASM_PATH"

# Read secret key
SECRET_KEY=$(cat "$SECRET_KEY_PATH" | tr -d '\n')

# Read WASM file and convert to base64
WASM_BASE64=$(base64 -i "$WASM_PATH" | tr -d '\n')

echo "🔑 Secret key loaded"
echo "📦 WASM file loaded ($(wc -c < "$WASM_PATH") bytes)"

# Get current state root hash
echo "📊 Getting current state..."
STATE_RESPONSE=$(curl -s -H "Authorization: $ACCESS_TOKEN" -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_get_state_root_hash","params":[],"id":1}' \
  "$NODE_URL")

echo "State response: $STATE_RESPONSE"

# Extract state root hash
STATE_ROOT_HASH=$(echo "$STATE_RESPONSE" | jq -r '.result.state_root_hash')

if [ "$STATE_ROOT_HASH" = "null" ]; then
  echo "❌ Failed to get state root hash"
  echo "$STATE_RESPONSE"
  exit 1
fi

echo "✅ State root hash: $STATE_ROOT_HASH"

# Create deploy using casper-client to generate the proper format
echo "🔨 Creating deploy..."
casper-client make-deploy \
  --session-path "$WASM_PATH" \
  --payment-amount 10000000000 \
  --chain-name "$CHAIN_NAME" \
  --secret-key "$SECRET_KEY_PATH" \
  --output /tmp/deploy.json

if [ $? -ne 0 ]; then
  echo "❌ Failed to create deploy"
  exit 1
fi

echo "✅ Deploy created"

# Read the deploy and format it properly for JSON-RPC
DEPLOY_TO_SEND=$(cat /tmp/deploy.json | jq '{jsonrpc: "2.0", method: "account_put_deploy", params: {deploy: .}, id: 1}')

echo "📤 Sending deploy..."
DEPLOY_RESPONSE=$(curl -s -H "Authorization: $ACCESS_TOKEN" -H "Content-Type: application/json" \
  -d "$DEPLOY_TO_SEND" \
  "$NODE_URL")

echo "Deploy response: $DEPLOY_RESPONSE"

# Extract deploy hash
DEPLOY_HASH=$(echo "$DEPLOY_RESPONSE" | jq -r '.result.deploy_hash')

if [ "$DEPLOY_HASH" = "null" ]; then
  echo "❌ Failed to deploy"
  echo "$DEPLOY_RESPONSE"
  exit 1
fi

echo "✅ Contract deployed successfully!"
echo "🔗 Deploy hash: $DEPLOY_HASH"
echo "🌐 View on testnet explorer: https://testnet.cspr.live/transaction/$DEPLOY_HASH"

# Clean up
rm -f /tmp/deploy.json
