# Ledger Dev Build Plane Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 190 | ship |
| stress | diagnostic quality | 215 | ship |
| edge | review cost | 138 | watch |
| recovery | safe rewrite | 215 | ship |
| stale | change width | 220 | ship |

Start with `stale` and `edge`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`stale` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
