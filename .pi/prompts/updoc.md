---
description: Update documentation to reflect current code implementation
---
Update documentation for: $ARGUMENTS

## Process

1. **Read the code:**
   - Identify all relevant source files (implementation, commands, tests)
   - Read each file in full to understand current behavior
   - Note key functionality, commands, options, and configuration

2. **Read the existing documentation:**
   - Find the relevant doc file in `docs/`
   - Understand the current documentation structure and style

3. **Analyze gaps:**
   - Compare code implementation with current documentation
   - Identify missing features, commands, or options
   - Note outdated information or incorrect examples
   - Check if tests reveal additional behavior to document

4. **Update the documentation:**
   - Keep it compact and concise (no fluff or unnecessary info)
   - Start with 1-2 line description of what the feature does
   - Include only essential information users need
   - Use clear, practical examples from actual code/tests
   - Follow markdown structure: Description → Usage → Details
   - Maintain consistent style with other kb docs
   - **DO NOT add Implementation/Code sections** - remove them if they exist

5. **Verify completeness:**
   - All commands/options documented
   - Examples are accurate and test-verified
   - No implementation details that users don't need to know
   - Environment variables and special behaviors covered

## Style Guidelines

- **Compact:** No verbose explanations, get to the point
- **Practical:** Show actual commands and examples
- **Accurate:** Reflect what the code actually does
- **Minimal:** Only document what users need to know
- **User-focused:** DO NOT include implementation details, struct definitions, or "In code" sections
- **Consistent:** Match the style of existing kb documentation
