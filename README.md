rust bindings for Stripe
========================

# Preparing your build environment

stripe-rust depends on a number of upstream libraries. Until [crate][crate] is
ready for the main stage, I've submoduled in compatible versions.

`make ext` ought to do something reasonable, as far as preparing them goes, but
isn't a dependency by default to allow for other configurations.
