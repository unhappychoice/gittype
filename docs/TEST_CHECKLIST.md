# GitType Manual Test Checklist

## 1. Title Screen

### Difficulty Selection
- [x] `←`/`h` to select left difficulty
- [x] `→`/`l` to select right difficulty
- [x] Shows 5 difficulties (Easy, Normal, Hard, Wild, Zen)

### Game Start
- [x] Space key starts game
- [x] Shows error when no challenges available

### Menu Navigation
- [x] `R` opens records screen
- [x] `A` opens analytics screen
- [x] `S` opens settings screen
- [x] `I`/`?` opens help screen
- [x] `Esc` exits application
- [x] `Ctrl+C` exits application

---

## 2. Typing Screen

### Before Start
- [x] Space starts countdown
- [x] `Esc` shows dialog
- [x] `S` in dialog skips stage
- [x] `Q` in dialog goes to failure screen
- [x] `Esc` in dialog closes it

### During Typing
- [x] Code content displays correctly
- [x] Mistakes highlighted in red
- [x] Cursor position accurate
- [x] Real-time stats display

### Stage Progression
- [x] Challenge completion advances to next stage
- [x] All stages complete goes to summary

---

## 3. Stage Summary Screen

- [x] Stage number displays
- [x] Score/WPM/Accuracy displays
- [x] Space advances to next stage
- [x] `R` restarts stage

---

## 4. Session Summary Screen

### Display
- [x] Total score displays
- [x] WPM/Accuracy displays
- [x] Rank displays

### Navigation
- [x] `R` retries session
- [x] `T` returns to title
- [x] `S` opens share screen
- [x] `D` opens details dialog
- [x] `Esc` exits

---

## 5. Session Share Screen

- [x] Result preview displays
- [x] `1` shares to X (Twitter)
- [x] `2` shares to Reddit
- [x]`3` shares to LinkedIn
- [x] `4` shares to Facebook
- [x] `Esc` returns back

---

## 6. Total Summary Screen

- [x] Cumulative stats display
- [x] Total score/keystrokes display
- [x] `S` opens share screen
- [x] `Esc` returns to title

---

## 7. Total Summary Share Screen (Bug Fixed)

- [x] **Score displays correctly** ← Bug fix
- [x] `1`-`4` shares to platforms
- [x] `Esc` returns back

---

## 8. Records Screen

### Filters
- [x] Period filter works (All/7d/30d/90d)
- [x] Sort toggle works (Date/Score/Repo/Duration)
- [x] Asc/Desc toggle works

### Navigation
- [x] `↑`/`↓` moves through list
- [x] Enter opens detail screen
- [x] `Esc` returns to title

---

## 9. Session Detail Screen

- [x] Session info displays
- [x] Each stage result displays
- [x] `Esc` returns back

---

## 10. Analytics Screen

### View Toggle
- [x] `←`/`→` switches views
- [x] Overview/Trends/Repositories/Languages

### Navigation
- [x] `↑`/`↓` moves through list
- [x] `Esc` returns to title

---

## 11. Settings Screen

### Color Mode
- [x] Dark/Light toggle works
- [x] Preview updates immediately

### Theme
- [x] Theme list displays
- [x] Theme selection previews
- [x] Enter saves settings
- [x] `Esc` cancels changes

---

## 12. Help Screen

- [x] All sections display
- [x] `←`/`→` switches sections
- [x] `↑`/`↓` scrolls content
- [x] `Esc` closes screen

---

## 13. Loading Screen

- [x] Progress displays
- [x] File names display
- [x] Spinner animates
- [x] `Ctrl+C` cancels

---

## 14. Trending Feature

### Language Selection
- [x] Language list displays
- [x] `↑`/`↓` selects
- [x] Enter confirms

### Repository Selection
- [x] Trending repos display
- [x] `↑`/`↓` selects
- [x] Enter starts game

---

## 15. Repository Management

- [x] `gittype repo list` shows list
- [x] `gittype repo play` shows selection
- [x] `gittype repo clear` clears cache

---

## 16. Cache Feature (Bug Fixed)

- [x] **Challenge cache saves to `.config/cache/`** ← Bug fix
- [x] **Trending cache saves to `.config/cache/`** ← Bug fix
- [x] Cache reuse speeds up loading
- [x] `gittype cache stats` shows stats
- [x] `gittype cache clear` clears cache

---

## 17. CLI Commands

- [x] `gittype` uses current directory
- [x] `gittype /path` uses specified path
- [x] `gittype --repo owner/repo` clones GitHub repo
- [x] `gittype --langs rust,python` filters languages
- [x] `gittype trending` opens trending
- [x] `gittype --help` shows help
- [x] `gittype --version` shows version

---

## 18. Error Handling

- [x] Invalid repo shows error
- [x] Non-existent path shows error
- [x] Network error shows appropriate message
