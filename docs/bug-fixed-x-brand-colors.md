# bug fixed: X share buttons now use official black-and-white brand colors

## Problem

The X share buttons were still using a blue background inherited from old Twitter-era styling.

That conflicted with current X branding and made the CTA look visually inconsistent with the X icon and wording.

## Verification

Checked the official X brand guidelines from `about.x.com`.

The guideline states:

1. The logo should be white on a black background or black on a white background.
2. The primary colors are:
   - `X Black` `#000000`
   - `X White` `#FFFFFF`

Source:

- https://about.x.com/content/dam/about-twitter/x/brand-toolkit/x-brand-guidelines.pdf

## Fix

Updated the X share buttons to use the official black/white palette:

1. `web/src/pages/u/[login].astro`
2. `web/src/pages/share/[githubId].astro`

The buttons now use:

- background `#000000`
- text/icon `#FFFFFF`
- hover background `#111111`

## Result

All visible X share CTAs now match the official X brand direction instead of the old blue Twitter color.
