ifeq ($(origin FEDORA_RELEASE), undefined)
else
  FEDORA_RELEASE_ARGS = --release=${FEDORA_RELEASE}
endif

ifeq ($(origin MANIFEST_PATH), undefined)
else
  MANIFEST_PATH_ARGS = --manifest-path=${MANIFEST_PATH}
endif

ifeq ($(origin CLIPPY_FIX), undefined)
  CLIPPY_OPTS = --all-targets --no-deps
else
  CLIPPY_OPTS = --fix
endif

DENY = -D warnings -D future-incompatible -D unused -D rust_2018_idioms -D nonstandard_style

CLIPPY_DENY = -D clippy::all -D clippy::cargo -A clippy::multiple-crate-versions

clippy:
	RUSTFLAGS="${DENY}" cargo clippy ${CLIPPY_OPTS} -- ${CLIPPY_DENY}

COMPARE_FEDORA_VERSIONS ?=
test-compare-fedora-versions:
	echo "Testing that COMPARE_FEDORA_VERSIONS environment variable is set to a valid path"
	test -e "${COMPARE_FEDORA_VERSIONS}"

check-fedora-versions: test-compare-fedora-versions
	${COMPARE_FEDORA_VERSIONS} ${MANIFEST_PATH_ARGS} ${FEDORA_RELEASE_ARGS} ${IGNORE_ARGS}

yamllint:
	yamllint --strict .github/workflows/*.yml

audit:
	cargo audit -D warnings


.PHONY:
	audit
	check-fedora-versions
	clippy
	test-compare-fedora-versions
	yamllint
