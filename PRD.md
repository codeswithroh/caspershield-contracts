# Smart Contract PRD

## Project: **CasperShield**

### Contract Name

`CasperShieldVault`

---

## 1. Objective

CasperShield is a smart contract that enforces **predefined safety modes** on DeFi actions.

Users select a safety mode.
The contract deterministically **allows or blocks transactions** based on that mode.

Users never define rules.
Rules are embedded in the contract.

---

## 2. Design Principles

* Consumer-first abstraction
* Deterministic behavior
* Minimal surface area
* Explicit failure reasons
* Hackathon-friendly scope

---

## 3. Non-Goals (Explicit)

* No AI or rule generation
* No governance or voting
* No contract upgrades
* No dynamic policies
* No price oracles
* No batching or automation

---

## 4. Safety Modes

### Enumeration

```
SAFE = 0
BALANCED = 1
DEGENERATE = 2
```

---

## 5. Storage Layout

### 5.1 User Safety Mode

```
user_mode: Map<AccountHash, SafetyMode>
```

If not set, default is `SAFE`.

---

### 5.2 Allowed Protocols

```
allowed_contracts: Set<ContractHash>
```

Used by SAFE and BALANCED modes.

---

### 5.3 Transaction Limits

```
max_tx_amount_safe: U512
max_tx_amount_balanced: U512
```

Hard limits enforced at execution time.

---

### 5.4 Admin

```
admin: AccountHash
```

Admin is set at deployment.

---

## 6. Constructor

### `init(admin, safe_limit, balanced_limit, allowed_contracts[])`

Initializes:

* Admin account
* SAFE transaction limit
* BALANCED transaction limit
* Initial allowed contract list

Callable only once.

---

## 7. Public Entry Points

### 7.1 `set_mode(mode: SafetyMode)`

**Purpose**
Allows a user to select their desired safety mode.

**Rules**

* Any user may call
* Overwrites previous value
* Invalid modes revert

**State Changes**

* Updates `user_mode[caller]`

**Events**

* `ModeChanged(caller, mode)`

---

### 7.2 `execute_action(target_contract, amount, action_payload)`

This is the **core enforcement entry point**.

**Inputs**

```
target_contract: ContractHash
amount: U512
action_payload: Bytes
```

**Execution Flow**

1. Identify caller
2. Read caller’s safety mode
3. Enforce mode rules
4. If valid, call `target_contract`
5. If invalid, revert with reason

---

## 8. Mode Enforcement Rules

### 8.1 SAFE Mode

All conditions must pass:

* `target_contract` ∈ `allowed_contracts`
* `amount <= max_tx_amount_safe`

Failure results in revert.

---

### 8.2 BALANCED Mode

Conditions:

* `amount <= max_tx_amount_balanced`

If `target_contract` not in `allowed_contracts`:

* Allow execution
* Emit warning event

---

### 8.3 DEGENERATE Mode

Conditions:

* None

Always execute.

---

## 9. Error Codes

These must be deterministic and explicit.

```
ERR_UNAUTHORIZED
ERR_CONTRACT_NOT_ALLOWED
ERR_AMOUNT_EXCEEDS_LIMIT
ERR_INVALID_MODE
```

---

## 10. Events

### `ModeChanged`

```
(user: AccountHash, mode: SafetyMode)
```

---

### `ActionBlocked`

```
(user: AccountHash, mode: SafetyMode, reason: ErrorCode)
```

Emitted immediately before revert.

---

### `ActionExecuted`

```
(user: AccountHash, target_contract: ContractHash, amount: U512, mode: SafetyMode)
```

---

### `ActionWarning`

```
(user: AccountHash, target_contract: ContractHash)
```

Emitted in BALANCED mode when interacting with non-allowlisted contracts.

---

## 11. Admin Entry Points

### 11.1 `add_allowed_contract(contract_hash)`

* Admin only

---

### 11.2 `remove_allowed_contract(contract_hash)`

* Admin only

---

### 11.3 `update_limits(safe_limit, balanced_limit)`

* Admin only

---

## 12. Security Assumptions

* Contract is non-upgradeable
* Admin is trusted (hackathon scope)
* All validations happen before external calls
* No reentrancy assumptions beyond Casper execution model

---

## 13. Testing Requirements

### Unit Tests

* Default mode is SAFE
* Mode switching works
* SAFE blocks:

  * non-allowlisted contracts
  * excessive amount
* BALANCED blocks:

  * excessive amount
* BALANCED emits warning for non-allowlisted contracts
* DEGENERATE allows all actions

---

### Integration Tests

* Blocked action with correct error
* Successful execution after mode change

---

## 14. Demo Flow (Required)

1. User in SAFE mode
2. Attempt unsafe action → blocked
3. Switch to DEGENERATE
4. Same action → succeeds

---

## 15. Implementation Notes (for AI IDE)

* Use Odra framework
* Single contract
* Explicit enums and error codes
* No abstraction layers
* Keep logic flat and readable

---

## 16. Deliverable

One deployed contract:

* Name: `CasperShieldVault`
* Network: Casper Testnet
* Deterministic behavior
* Fully demoable

---
