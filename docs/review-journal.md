# Review Journal

The cases below are the review handles I would use before changing the implementation.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 190, lane `ship`
- `stress`: `diagnostic quality`, score 215, lane `ship`
- `edge`: `review cost`, score 138, lane `watch`
- `recovery`: `safe rewrite`, score 215, lane `ship`
- `stale`: `change width`, score 220, lane `ship`

## Note

A future change should add new cases before it changes the scoring rule.
