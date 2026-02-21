# Todo

## vault.rs

- [ ] Add `description: Option<String>` to `Domain` (see docs/domain-description.md)
- [ ] Remove redundant `filename` field from `Note` (derivable from `path.file_name()`)
- [ ] Make excluded domain patterns configurable (currently hardcoded: dirs starting with `.` or `_`)

## tags

- [ ] Implement `kbase tags --domain <domain>` to filter tags by domain (uses `TagIndex::filter_by_domains()`)
