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

ifeq ($(origin MINIMAL), undefined)
  BUILD = build
else
  BUILD = minimal-versions build --direct
endif

clippy:
	cargo clippy ${CLIPPY_OPTS}

COMPARE_FEDORA_VERSIONS ?=
test-compare-fedora-versions:
	echo "Testing that COMPARE_FEDORA_VERSIONS environment variable is set to a valid path"
	test -e "${COMPARE_FEDORA_VERSIONS}"

check-fedora-versions: test-compare-fedora-versions
	${COMPARE_FEDORA_VERSIONS} ${MANIFEST_PATH_ARGS} ${FEDORA_RELEASE_ARGS} ${IGNORE_ARGS}

yamllint:
	yamllint --strict .github/workflows/*.yml
fmt:
	cargo fmt

fmt-ci:
	cargo fmt --all -- --check

audit:
	cargo audit -D warnings

build:
	cargo ${BUILD}

.PHONY:
	audit
	build
	check-fedora-versions
	clippy
	fmt
	fmt-ci
	test-compare-fedora-versions
	yamllint
