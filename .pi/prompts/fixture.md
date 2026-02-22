---
description: Test kbase against fixture vault in isolated /tmp setup
---

Set up kbase with fixture vault for manual testing:

```bash
# Create isolated config
cd /tmp && rm -rf test-kbase && mkdir test-kbase

# Add fixture vault
KBASE_HOME=/tmp/test-kbase/.kbase kbase add test-vault /Users/balu/Code/kbase/tests/fixtures/vault

# Use it
export KBASE_HOME=/tmp/test-kbase/.kbase
kbase domains
kbase notes --domain lucene
kbase read lucene/search-flow.md
```

This lets you see actual output and iterate quickly without test rebuild cycles.

$ARGUMENTS
