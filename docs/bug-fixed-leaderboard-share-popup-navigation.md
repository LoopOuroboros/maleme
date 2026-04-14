# bug fixed: leaderboard share popup no longer redirects the current page to X

## Problem

The leaderboard share button on the home page tried to open X in a new tab, but it also had a fallback:

1. call `window.open(...)`
2. if that returned a falsy value, redirect the current page with `window.location.href`

That meant if the popup was blocked or treated as failed by the browser, the current leaderboard page was replaced by X.

## Fix

Updated `web/src/components/LeaderboardTable.astro`:

1. removed the current-page redirect fallback
2. kept the share action as a new-tab popup only
3. cleared the popup opener reference when the popup is created

## Result

Clicking the leaderboard share action no longer turns the current page into X. The current page stays where it is.
