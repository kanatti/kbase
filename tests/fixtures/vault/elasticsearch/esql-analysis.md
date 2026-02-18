# ES|QL Analysis Phase

Deep dive into the ES|QL parsing and analysis pipeline.

This is a #deep-dive analysis of #elasticsearch #query-parsing. Status: #wip.

## Overview

ES|QL is a new query language for Elasticsearch built on top of a dedicated
parser and analyzer.

```sql
-- This is #fake-tag in code, should be ignored
FROM employees | WHERE department = "#also-fake"
```

The implementation uses #antlr for parsing and has #performance implications.
