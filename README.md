# ledger-dev-build-plane

`ledger-dev-build-plane` is a Rust project in developer tools. Its focus is to build a Rust toolkit that studies build behavior through node-edge fixtures, with cycle and reachability reports and no production deployment claims.

## Why This Exists

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how change width and review cost should influence a review result.

## Ledger Dev Build Plane Review Notes

The first comparison I would make is `change width` against `review cost` because it shows where the rule is most opinionated.

## Capabilities

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/ledger-dev-build-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `change width` and `review cost`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Implementation Shape

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The Rust addition stays small enough to inspect in one sitting.

## Local Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Verification

That command is also the regression path. It verifies the domain cases and catches mismatches between the CSV, metadata, and code.

## Roadmap

The repository is intentionally scoped to local checks. I would expand it by adding adversarial fixtures before adding features.
