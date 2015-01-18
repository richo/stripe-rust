[![Build Status](https://travis-ci.org/richo/stripe-rust.svg)](https://travis-ci.org/richo/stripe-rust)

rust bindings for Stripe
========================

# Preparing your build environment

stripe-rust depends on a number of upstream libraries. Until [crate][crate] is
ready for the main stage, I've submoduled in compatible versions.

`make ext` ought to do something reasonable, as far as preparing them goes, but
isn't a dependency by default to allow for other configurations.

# Stability

stripe-rust is tracking rust master, on a totally ad-hoc "As I happen to rebuild rust" fashion.

The last knowngood revision is: mozilla/rust@59f4619


# Run make locally as a developer
```make -L ext```