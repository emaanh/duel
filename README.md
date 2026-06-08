# duel - a terminal tool for ranking anything

![The Elo formula, as seen in The Social Network](imgs/elo_sn.gif)

In my favorite movie, *The Social Network*, Mark Zuckerberg builds Facemash, a tasteless but effective demonstration of ranking elements through 1v1 duels. He used the Elo rating system (shown in the gif above), the same rating system that ranks chess players. This was my introduction to pairwise comparison, and since then I've used it to cut through noise when ranking elements.

Before making `duel`, I used websites that do this. But I wanted a system-wide tool where I could save my sessions, and do it from the comfort of my terminal without opening a browser. `duel` is that.

Give it a list. It pits items against each other one at a time, updates Elo scores after each pick, and builds a ranking that reflects your actual preferences. Sessions are saved automatically so you can quit and come back later.

---

## Install

```bash
git clone https://github.com/emaanh/duel.git
cd duel
cargo install --path .
```

---

## Usage

```
duel <file>                           # start or resume a session
duel <file> --results                 # show current rankings without dueling
duel <file> --results -o out.txt      # save rankings to a file
```

Items are read from `<file>`, one per line. Blank lines are ignored. Sessions are saved automatically to `~/.local/share/duel/` after every pick.

**During a duel:**

```
  ████████░░░░░░░░░░░░ 12/40 (30%)  •  least-seen item: 2 duels
─────────────────────────────────────────
  [1]  Inception

            vs

  [2]  The Dark Knight
─────────────────────────────────────────
  Pick (1/2/s/q):
```

- `1` or `2` to pick the winner
- `s` to skip
- `q` to quit (progress is already saved)

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

Each item starts at **1000 Elo**. After every matchup, the winner gains points and the loser loses points. The amount depends on how surprising the result was: an upset earns more than a win everyone expected.

Matchmaking is adaptive. Items with fewer comparisons get prioritized first, then matched against whoever has the closest Elo score. The ranking gets smarter faster, and you can see your progress toward a complete ranking with every matchup.
