# TODO: Category Front Matter Normalisation

**Priority**: 🟡
**Estimated Effort**: 1-2 days
**Created**: 2025-09-22
**Status**: Completed
**Completed**: 2025-01-11

## Problem Description

Many posts declare a `category` or `categories` field in their front matter, but the values are inconsistent. Some posts expose arrays, some use placeholder values like `post`/`posts`, and others mix legacy keys. This prevents downstream features from relying on the metadata because the semantics are unclear and forces templates to handle multiple shapes. To ship reliable category-based navigation, we need a single, meaningful category value or nothing at all.

## Proposed Solution

- Edit every post listed below so its front matter exposes at most one meaningful `category` value: trim whitespace, collapse arrays to the first non-empty value, and normalise casing while you update the file.
- Remove generic placeholder values (e.g., `post`, `posts`) entirely so posts without a meaningful category simply omit the field.
- Record any posts that lost their category during clean-up so follow-up editorial decisions can be made (e.g., notes in commit message or a tracking doc).
- Optionally add a lightweight check (script or documentation snippet) that future posts should avoid array syntax or placeholder categories.

## Implementation Plan

### Step 1: Finalise inventory and heuristics
- Review the lists below and confirm the handling for edge cases (arrays, placeholders, empty strings).
- Expected outcome: Signed-off rules for normalisation and removal.
- Dependencies or prerequisites: None.

### Step 2: Update post front matter
- Apply the agreed-upon normalisation rules to each post file, ensuring the `category` field is either a single meaningful string or removed.
- Expected outcome: Repository content embeds the normalised category metadata.
- Dependencies or prerequisites: Step 1 alignment on heuristics.

### Step 3: Document outcomes
- Capture which posts had categories removed or changed (commit message, changelog, or editorial note) so stakeholders can review.
- Expected outcome: Clear audit trail of the edits and any open follow-ups for content owners.
- Dependencies or prerequisites: Step 2 complete.

### Step 4: Add guardrails for new content
- Update documentation or content guidelines, and optionally add a simple lint script, so new posts follow the single-category rule.
- Expected outcome: Future posts ship with compliant front matter without relying on code-side fixes.
- Dependencies or prerequisites: Steps 1-3 define the desired behaviour.

## Success Criteria

- [x] All posts define at most one meaningful `category` value (or none) in their front matter.
- [x] Placeholder categories (`post`/`posts`) are removed from the repository.
- [x] Guidance or tooling exists to prevent reintroducing arrays or placeholders in new posts.

## Affected Components

- `sites/*/posts/*.md` - Manually normalise the front matter values.
- Documentation / contributor guidelines - Clarify the single-category rule.

## Risks & Considerations

- **Risk**: Manual edits may introduce front matter typos; mitigate with careful review or scripted validation.
- **Risk**: Removing placeholder values might hide posts from category listings until meaningful categories are added; track affected files for editorial follow-up.
- **Dependencies**: Blocks pagination work tracked in `todo/012-category-pagination-pages.md`.
- **Breaking Changes**: None expected in code, but content diffs should be reviewed closely.

## Related Items

- **Blocks**: `todo/012-category-pagination-pages.md`
- **Depends on**: None.
- **Related**: Existing front matter parsing tests and pagination logic.

## References

- Site content under `sites/*/posts` demonstrating current `categories` usage.
- Existing pagination generator that will consume the normalised categories.

## Notes

- Consider running a one-off script to verify no `categories:` arrays remain after edits.
- Once normalisation is in place, we can consider schema validation for front matter to prevent regressions.

## Posts with `categories` Front Matter

These posts currently declare a `categories` field and should be validated during normalisation:

### Active
- sites/polgarand.org/posts/2015-11-15-route66-roadtrip.md
- sites/polgarand.org/posts/2016-10-02-eszter-endre.md
- sites/polgarand.org/posts/2016-11-03-berlin.md
- sites/polgarand.org/posts/2016-11-15-luxembourg.md
- sites/polgarand.org/posts/2016-12-25-happy-holidays.md
- sites/polgarand.org/posts/2019-04-20-north-korea.md
- sites/polgarand.org/posts/2019-08-05-viktoriia.md
- sites/polgarand.org/posts/2019-08-07-oksi.md
- sites/polgarand.org/posts/2019-08-30-jan-and-nick.md
- sites/polgarand.org/posts/2019-08-31-summer-collector.md
- sites/polgarand.org/posts/2019-09-17-budapest-judit.md
- sites/polgarand.org/posts/2019-10-01-sophy.md
- sites/polgarand.org/posts/2019-10-04-dora-and-peter.md
- sites/polgarand.org/posts/2019-10-06-miyu.md
- sites/polgarand.org/posts/2019-11-04-frida.md
- sites/polgarand.org/posts/2019-11-24-luxembourg.md
- sites/polgarand.org/posts/2020-02-19-sydney.md
- sites/polgarand.org/posts/2020-03-06-alina.md
- sites/polgarand.org/posts/2020-03-07-new-zealand.md
- sites/polgarand.org/posts/2020-05-26-texel.md
- sites/polgarand.org/posts/2020-06-06-kerry.md
- sites/polgarand.org/posts/2020-08-23-mia.md
- sites/polgarand.org/posts/2020-09-02-rome.md
- sites/polgarand.org/posts/2020-09-13-emma.md
- sites/polgarand.org/posts/2020-09-21-martina.md
- sites/polgarand.org/posts/2020-09-26-woutla.md
- sites/polgarand.org/posts/2020-10-04-stedelijk-museum.md
- sites/polgarand.org/posts/2020-10-28-olive-rotterdam.md
- sites/polgarand.org/posts/2020-12-02-shibari-maya-asimira.md
- sites/polgarand.org/posts/2020-12-29-darkest-before-dawn.md
- sites/polgarand.org/posts/2021-02-20-ju-distorted-mirror.md
- sites/polgarand.org/posts/2021-03-25-noa.md
- sites/polgarand.org/posts/2021-03-29-covid19.md
- sites/polgarand.org/posts/2021-04-14-kate.md
- sites/polgarand.org/posts/2021-05-30-elysian.md
- sites/polgarand.org/posts/2021-08-01-noa.md
- sites/polgarand.org/posts/2021-08-07-julia-and-diego.md
- sites/polgarand.org/posts/2021-08-24-layana.md
- sites/polgarand.org/posts/2021-08-28-boating-in-haarlem.md
- sites/polgarand.org/posts/2021-09-03-amsterdam-vienna-nightjet.md
- sites/polgarand.org/posts/2021-09-09-petra.md
- sites/polgarand.org/posts/2021-09-10-rebeka.md
- sites/polgarand.org/posts/2021-09-19-david-budapest.md
- sites/polgarand.org/posts/2021-09-19-kermis-beverwijk.md
- sites/polgarand.org/posts/2021-09-25-andor-30.md
- sites/polgarand.org/posts/2021-10-09-the-hague.md
- sites/polgarand.org/posts/2021-11-01-halloween-in-hungary.md
- sites/polgarand.org/posts/2021-11-04-peyton.md
- sites/polgarand.org/posts/2021-11-25-lisbon.md
- sites/polgarand.org/posts/2021-12-11-marcelo.md
- sites/polgarand.org/posts/2022-01-15-ju-and-leo.md
- sites/polgarand.org/posts/2022-01-15-shaving-ju.md
- sites/polgarand.org/posts/2022-01-30-sick-ducks-digital.md
- sites/polgarand.org/posts/2022-01-30-sick-ducks-film.md

### Archived
- sites/polgarand.org/_archived/2014-08-01-paris.md
- sites/polgarand.org/_archived/2014-08-29-barcelona.md
- sites/polgarand.org/_archived/2015-01-12-trier-winter.md
- sites/polgarand.org/_archived/2015-05-30-erasmus-bridge.md
- sites/polgarand.org/_archived/2015-08-06-athens.md
- sites/polgarand.org/_archived/2015-09-24-budapest.md
- sites/polgarand.org/_archived/2015-11-21-scanners-scanning.md
- sites/polgarand.org/_archived/2016-01-03-mudam.md
- sites/polgarand.org/_archived/2016-03-09-dakimakura.md
- sites/polgarand.org/_archived/2016-03-09-five-g-tokyo.md
- sites/polgarand.org/_archived/2016-03-10-tokyo-sex.md
- sites/polgarand.org/_archived/2016-03-11-tokyo.md
- sites/polgarand.org/_archived/2016-03-26-revision.md
- sites/polgarand.org/_archived/2016-05-05-synth-cave.md
- sites/polgarand.org/_archived/2016-05-11-budapest.md
- sites/polgarand.org/_archived/2016-06-11-sold-my-doepfer-case.md
- sites/polgarand.org/_archived/2016-06-12-elite-modular.md
- sites/polgarand.org/_archived/2016-06-22-fete-nationale-de-luxembourg.md
- sites/polgarand.org/_archived/2016-07-25-wim-delvoye-in-luxembourg.md
- sites/polgarand.org/_archived/2016-08-12-sofar-sounds.md
- sites/polgarand.org/_archived/2016-09-03-manufactured-in-los-angeles.md
- sites/polgarand.org/_archived/2016-09-10-san-diego.md
- sites/polgarand.org/_archived/2016-10-01-erica-synths-wavetable-vco.md
- sites/polgarand.org/_archived/2016-10-06-locked-myself-out.md
- sites/polgarand.org/_archived/2016-10-08-marina-del-rey.md
- sites/polgarand.org/_archived/2016-10-22-batumi.md
- sites/polgarand.org/_archived/2016-10-22-perfect-circuit-audio.md
- sites/polgarand.org/_archived/2016-10-31-schneidersladen.md
- sites/polgarand.org/_archived/2016-11-24-quadnic.md
- sites/polgarand.org/_archived/2016-11-24-synthesizing-los-angeles.md
- sites/polgarand.org/_archived/2016-12-29-jellies-of-csc.md
- sites/polgarand.org/_archived/2017-01-20-namm.md
- sites/polgarand.org/_archived/2017-01-22-ambient-eurorack.md
- sites/polgarand.org/_archived/2017-02-15-marble-macbook.md
- sites/polgarand.org/_archived/2017-03-04-amsterdam.md
- sites/polgarand.org/_archived/2017-03-12-amsterdam.md
- sites/polgarand.org/_archived/2017-04-09-haarlem.md
- sites/polgarand.org/_archived/2017-04-15-broken-radio.md
- sites/polgarand.org/_archived/2017-04-23-superbooth.md
- sites/polgarand.org/_archived/2017-04-23-zuid-kennemerland-national-park.md
- sites/polgarand.org/_archived/2017-06-10-new-life-new-eurorack-case.md
- sites/polgarand.org/_archived/2017-07-01-modular-festival-colin-benders.md
- sites/polgarand.org/_archived/2017-07-14-vactrol-drums.md
- sites/polgarand.org/_archived/2017-07-17-helios-40-2.md
- sites/polgarand.org/_archived/2017-07-22-modular-meditation.md
- sites/polgarand.org/_archived/2017-08-06-helios-lens-flare-porn.md
- sites/polgarand.org/_archived/2017-08-13-portrait-shooting-with-helios.md
- sites/polgarand.org/_archived/2017-08-18-isco-cinemascope-anamorphic-portrait.md
- sites/polgarand.org/_archived/2017-09-06-visiting-erica-synths-hq.md
- sites/polgarand.org/_archived/2017-09-08-riga.md
- sites/polgarand.org/_archived/2017-09-10-kaunas.md
- sites/polgarand.org/_archived/2017-09-17-anamorphic-test-shoot.md
- sites/polgarand.org/_archived/2017-09-19-susanne-sundfor.md
- sites/polgarand.org/_archived/2017-09-21-los-angeles.md
- sites/polgarand.org/_archived/2017-09-24-everglades-florida.md
- sites/polgarand.org/_archived/2017-09-28-miami.md
- sites/polgarand.org/_archived/2017-10-09-home-is-where-my-rhodes-is.md
- sites/polgarand.org/_archived/2017-10-27-bratislava.md
- sites/polgarand.org/_archived/2017-11-05-haarlem.md
- sites/polgarand.org/_archived/2017-11-23-kaitlyn-aurelia-smith.md
- sites/polgarand.org/_archived/2017-12-09-my-little-helper.md
- sites/polgarand.org/_archived/2017-12-29-sopron.md
- sites/polgarand.org/_archived/2018-01-07-budapest.md
- sites/polgarand.org/_archived/2018-01-08-atreyus-horse-andor-polgar.md
- sites/polgarand.org/_archived/2018-01-14-irbene-radio-telescope-video.md
- sites/polgarand.org/_archived/2018-02-02-nils-frahm-paradios-amsterdam.md
- sites/polgarand.org/_archived/2018-02-08-freelensing.md
- sites/polgarand.org/_archived/2018-02-19-noodlebar-rotterdam.md
- sites/polgarand.org/_archived/2018-02-26-susanne-sundfor.md
- sites/polgarand.org/_archived/2018-03-10-kaaswinkeltje.md
- sites/polgarand.org/_archived/2018-03-18-ear-session.md
- sites/polgarand.org/_archived/2018-04-04-eniko-haarlem.md
- sites/polgarand.org/_archived/2018-04-06-slr-magic-amsterdam.md
- sites/polgarand.org/_archived/2018-04-07-sofie.md
- sites/polgarand.org/_archived/2018-04-28-japan-random-wehicle-window.md
- sites/polgarand.org/_archived/2018-05-15-gion-girls.md
- sites/polgarand.org/_archived/2018-05-20-yoori.md
- sites/polgarand.org/_archived/2018-05-25-sofie.md
- sites/polgarand.org/_archived/2018-06-03-museum-van-geluid.md
- sites/polgarand.org/_archived/2018-06-10-poes-service-bell-dark-ambient.md
- sites/polgarand.org/_archived/2018-06-24-csilla-cinemascope.md
- sites/polgarand.org/_archived/2018-06-26-budapest.md
- sites/polgarand.org/_archived/2018-07-02-brighton-pier.md
- sites/polgarand.org/_archived/2018-07-02-brighton-suburb.md
- sites/polgarand.org/_archived/2018-07-03-brighton-modular-meet.md
- sites/polgarand.org/_archived/2018-07-29-tuscany.md
- sites/polgarand.org/_archived/2018-07-30-nudes.md
- sites/polgarand.org/_archived/2018-08-20-rhodes.md
- sites/polgarand.org/_archived/2018-09-30-dutch-modular-fest.md
- sites/polgarand.org/_archived/2018-10-15-helgoland.md
- sites/polgarand.org/_archived/2018-10-20-schiermonnikoog.md
- sites/polgarand.org/_archived/2018-12-22-luxembourg.md
- sites/polgarand.org/_archived/2019-01-01-happy-new-year.md
- sites/polgarand.org/_archived/2019-01-14-andor-polgar-paradiso.md
- sites/polgarand.org/_archived/2019-01-21-budapest.md
- sites/polgarand.org/_archived/2019-02-17-david.md
- sites/polgarand.org/_archived/2019-02-20-kamilla.md
- sites/polgarand.org/_archived/2019-03-02-mwc-barcelona.md
- sites/polgarand.org/_archived/2019-05-19-waveform-research-centre.md

## Completion Summary

**Completed on**: 2025-01-11

**Changes Made**:
- Normalized 153 posts across active and archived content
- Removed all placeholder values (`post`, `posts`) from active posts  
- Converted array syntax `[category]` to singular `category: value` in archived posts
- Archived posts now have meaningful categories: `music`, `cinemascope`, `travel`, `artnude`
- Active posts with placeholder categories now have no category field (as intended)

**Files Added**:
- `scripts/validate_categories.py` - Validation script to prevent regressions
- `docs/category-guidelines.md` - Documentation for contributors

**Validation**: All tests pass and validation script confirms no invalid category usage remains.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
