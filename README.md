# duel

![The Elo formula, as seen in The Social Network](imgs/elo_sn.gif)

In *The Social Network*, Mark Zuckerberg builds Facemash in a single night — a site that puts two things side by side and asks one question: *which one is better?* He used Elo, the same rating system that ranks chess grandmasters. The idea was simple and kind of perfect: pairwise comparison cuts through noise and forces real opinions.

`duel` is that idea, for anything. Give it a list. It pits items against each other one at a time, updates Elo scores after each pick, and builds a ranking that reflects your actual preferences — not just vibes.

---

## Usage

```
duel <file>                    # start or resume a session
duel <file> --results          # show current rankings without dueling
duel <file> --results -o rankings.txt  # save rankings to a file
```

Items are read from `<file>`, one per line. Blank lines are ignored. Progress is automatically saved alongside your input file so you can quit and pick up later.

**During a duel:**

```
─────────────────────────────────────────
  [1]  Inception

            vs

  [2]  The Dark Knight
─────────────────────────────────────────
  Pick (1/2/s/q):
```

- `1` or `2` — pick the winner
- `s` — skip this matchup
- `q` — quit and save progress

**Rankings output:**

```
Rankings  (42 comparisons total)

Rank  Score    W      L       Item
──────────────────────────────────────────────────
1     1089     8      2       The Dark Knight
2     1047     6      3       Inception
3     1021     5      4       Interstellar
4     978      3      6       Tenet
5     864      2      7       Dunkirk
```

---

## How it works

Each item starts at **1000 Elo**. After every matchup, the winner gains points and the loser loses points — the amount depends on how surprising the result was. An upset earns more than a win everyone expected.

Matchmaking is adaptive: items with fewer comparisons get prioritized first, then matched against whoever has the closest Elo score. This means the ranking gets smarter faster, and you don't waste comparisons on mismatches.

---

## Install

```bash
cargo install --path .
```
