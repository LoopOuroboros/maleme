# bug fixed: share page OG image now falls back to GitHub avatar

## Problem

The leaderboard share page was publishing its social preview image as an SVG URL.

That looked fine in our own implementation, but it broke card previews on X because X does not support SVG for `twitter:image`.

## Verification

Confirmed against the official X Cards markup reference:

- `twitter:image` supports `JPG`, `PNG`, `WEBP`, and `GIF`
- `SVG is not supported`

Source:

- https://developer.x.com/en/docs/x-for-websites/cards/overview/markup

## Fix

Updated the share page metadata in `web/src/pages/share/[githubId].astro`:

1. stopped using the generated SVG as the primary social image
2. temporarily use the user's GitHub avatar URL as the social preview image
3. apply the same raster image to both `og:image` and `twitter:image`
4. added `twitter:image:alt`

## Result

Sharing `/share/:githubId` links now has a raster image available for X cards and Open Graph consumers, so previews can render an image instead of showing no card image.
