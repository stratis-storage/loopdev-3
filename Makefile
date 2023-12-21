ifeq ($(origin CLIPPY_FIX), undefined)
  CLIPPY_OPTS = --all-targets --no-deps
else
  CLIPPY_OPTS = --fix
endif

DENY = -D warnings -D future-incompatible -D unused -D rust_2018_idioms -D nonstandard_style

CLIPPY_DENY = -D clippy::all -D clippy::cargo -A clippy::multiple-crate-versions

clippy:
	RUSTFLAGS="${DENY}" cargo clippy ${CLIPPY_OPTS} -- ${CLIPPY_DENY}
