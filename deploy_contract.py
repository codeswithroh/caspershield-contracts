#!/usr/bin/env python3
import requests
import json
import base64
import time

# Configuration
NODE_URL = "https://node.testnet.cspr.cloud/rpc"
ACCESS_TOKEN = "<access-token>"
CHAIN_NAME = "casper-test"
SECRET_KEY_PATH = "<secret-path>"
WASM_PATH = "<wasm-path>"

def deploy_contract():
    headers = {
        "Authorization": ACCESS_TOKEN,
        "Content-Type": "application/json"
    }
    
    print("🚀 Deploying CasperShield Contract to Testnet...")
    print(f"📡 Node: {NODE_URL}")
    print(f"🔗 Chain: {CHAIN_NAME}")
    print(f"📄 WASM: {WASM_PATH}")
    
    # Read WASM file
    with open(WASM_PATH, "rb") as f:
        wasm_bytes = f.read()
    
    # Read secret key
    with open(SECRET_KEY_PATH, "r") as f:
        secret_key = f.read().strip()
    
    # Get current state root hash
    print("📊 Getting current state...")
    state_response = requests.post(NODE_URL, headers=headers, json={
        "jsonrpc": "2.0",
        "method": "chain_get_state_root_hash",
        "params": [],
        "id": 1
    })
    
    if state_response.status_code != 200:
        print(f"❌ Failed to get state: {state_response.status_code}")
        print(state_response.text)
        return
    
    state_root_hash = state_response.json()["result"]["state_root_hash"]
    print(f"✅ State root hash: {state_root_hash}")
    
    # Create deploy
    timestamp = int(time.time() * 1000)
    ttl = "30m"
    
    deploy_json = {
        "jsonrpc": "2.0",
        "method": "account_put_deploy",
        "params": {
            "deploy": {
                "header": {
                    "account": secret_key,
                    "timestamp": timestamp,
                    "ttl": ttl,
                    "gas_price": 1,
                    "body_hash": "",  # Will be calculated
                    "dependencies": [],
                    "chain_name": CHAIN_NAME
                },
                "payment": {
                    "StoredValue": {
                        "StoredContract": {
                            "package_hash": "hash-0000000000000000000000000000000000000000000000000000000000000000",
                            "entry_point": "standard_payment",
                            "args": [
                                {
                                    "cl_type": "U512",
                                    "bytes": "0400e1f50500000000",  # 10000000000 in hex
                                    "parsed": "10000000000"
                                }
                            ]
                        }
                    }
                },
                "session": {
                    "StoredValue": {
                        "StoredContract": {
                            "package_hash": "hash-0000000000000000000000000000000000000000000000000000000000000000",
                            "entry_point": "call",
                            "args": []
                        }
                    }
                }
            }
        },
        "id": 2
    }
    
    print("📤 Sending deploy...")
    deploy_response = requests.post(NODE_URL, headers=headers, json=deploy_json)
    
    if deploy_response.status_code == 200:
        result = deploy_response.json()
        if "result" in result:
            deploy_hash = result["result"]["deploy_hash"]
            print(f"✅ Contract deployed successfully!")
            print(f"🔗 Deploy hash: {deploy_hash}")
            print(f"🌐 View on testnet explorer: https://testnet.cspr.cloud/deploy/{deploy_hash}")
        else:
            print(f"❌ Deploy failed: {result}")
    else:
        print(f"❌ Failed to deploy: {deploy_response.status_code}")
        print(deploy_response.text)

if __name__ == "__main__":
    deploy_contract()
