# bug fixed: share buttons now use X branding and icon

## Problem

The share flow was inconsistent across leaderboard pages:

1. Some buttons still said `Twitter` instead of `X`.
2. The wording mixed Chinese and English across share entry points.
3. X share buttons did not have a recognizable brand icon, so the action was less obvious.

## Fix

Updated the X share actions to use consistent branding:

1. `web/src/pages/u/[login].astro`
   - `Share on X` text
   - added X icon to the profile action and submit-success modal action

2. `web/src/pages/share/[githubId].astro`
   - replaced `Twitter` wording with `Share on X`
   - added X icon to the share button
   - aligned copy-link button text to English

3. `web/src/components/LeaderboardTable.astro`
   - added X icon to the leaderboard share button
   - kept the wider no-wrap button layout

## Result

All visible share actions now use `X` naming in English and display the X icon consistently.

## Files

- `web/src/components/LeaderboardTable.astro`
- `web/src/pages/u/[login].astro`
- `web/src/pages/share/[githubId].astro`
