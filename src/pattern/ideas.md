# Pattern ideas

Deferred notes.

## Semantics

- Partial captures: consider zeroing branch outputs that were not written.
  Insert those writes after flow analysis so reads before writes remain errors.
- `?` is an unchecked `skip(1)`. A validating wildcard atom can wait until a real boundary case calls for it.
- Reconsider whether cursors may remain in virtual, file-unbacked memory now that typed reads exist.
- `Pir` remains experimental and unexposed in pattern syntax.

## Scanner

- Review optimized-search range tails and section/file boundary assumptions later.
- Alternative scans can backtrack heavily. Patterns are trusted for now.

## Testing

- Differentially compare optimized scanning with direct execution at every RVA.
- Exercise empty input, final-byte cursors, section edges, nested retries, and reference returns.
- Add parser/compiler fuzzing when the syntax settles.

## Tooling

- Revisit `findsig` output and error handling separately.
