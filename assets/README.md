# Artwork and application icon

## Overview

The accepted digital identity lives in [brand/README.md](brand/README.md). The foundation also retains an earlier application icon; the presence of approved artwork does not establish finished product-wide integration.

## Interface

- [icon.svg](icon.svg) is the existing foundation icon source. The desktop bundle's [icon files](../src-tauri/icons/) are configured by [tauri.conf.json](../src-tauri/tauri.conf.json).
- The brand [editable master](brand/source/metten-master.svg), [specifications](brand/specifications.json) and [guide](brand/brand-guide.pdf) define the delivered digital kit.
- [SVG](brand/exports/svg/), [PNG](brand/exports/png/) and [PDF](brand/exports/pdf/) directories contain prepared variants; their role, palette, minimum sizes and provenance are documented in the brand README. All files in those directories belong to that static kit.
- [font-source.json](brand/licenses/font-source.json), [Clash license](brand/licenses/Clash-Grotesk-FFL.txt) and [Inter license](brand/licenses/Inter-OFL.txt) record font provenance; font software is not bundled.

## Invariants

The kit specifies cobalt `#304FFF`, ink `#15171C`, chalk `#F1EFE8`, outlined Clash Grotesk lettering and Inter interface type. Use micro symbols at 16–23 CSS pixels, small at 24–47 and main at 48 or larger; the full lockup's minimum visible width is 240 pixels. These are asset usage requirements, not a claim that the current foundation screen implements the final design. Do not infer trademark clearance, print certification or font redistribution rights from the software license.

## Side Effects

The artwork itself executes no application operations. Referencing it changes rendered appearance; bundle icons are copied into build output. No artwork was regenerated in this baseline task.

## Verification

Verify referenced asset paths and compare usage with the brand README and specifications. This documentation pass inventories static exports and reads their ownership/usage records; it does not rerender or certify every raster/PDF export. Product branding remains staged in the [implementation plan](../docs/implementation-plan.md).
