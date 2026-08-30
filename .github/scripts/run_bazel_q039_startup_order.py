"""Q0.39 canonical base-startup order for keyless Windows gnullvm."""

from __future__ import annotations

from collections.abc import Mapping, Sequence

from run_bazel_q028_startup_contract import DISABLED_REPO_CONTENTS_CACHE
from run_bazel_q028_startup_contract import OUTPUT_USER_ROOT_PREFIX


def canonicalize_keyless_windows_gnullvm_base_startup(
    startup: Sequence[str],
    env: Mapping[str, str],
) -> list[str]:
    """Return the one reviewed base-startup vector in authority order.

    The preserved compatibility wrapper prepends startup options it injects to
    options already supplied by ``run-bazel-ci-impl.sh``. The real CI caller
    supplies ``--output_user_root`` itself, so the compatibility result is
    ``noexperimental, output_user_root`` even though Q0.28 requires the reverse
    authority order. Accept only the two exact reviewed values, reject every
    drift or duplicate, and normalize their order before strict rc controls are
    appended and the complete Q0.28 vector is validated.
    """

    output_user_root = env.get("BAZEL_OUTPUT_USER_ROOT")
    if not output_user_root:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires "
            "BAZEL_OUTPUT_USER_ROOT"
        )
    expected_output_root = f"{OUTPUT_USER_ROOT_PREFIX}{output_user_root}"

    output_roots = [
        option for option in startup if option.startswith(OUTPUT_USER_ROOT_PREFIX)
    ]
    if output_roots != [expected_output_root]:
        raise ValueError(
            "credential-free Windows gnullvm exact startup vector requires exactly "
            f"{expected_output_root!r}; observed {output_roots!r}"
        )

    cache_controls = [
        option for option in startup if option == DISABLED_REPO_CONTENTS_CACHE
    ]
    if cache_controls != [DISABLED_REPO_CONTENTS_CACHE]:
        raise ValueError(
            "credential-free Windows gnullvm exact startup vector requires exactly one "
            f"{DISABLED_REPO_CONTENTS_CACHE!r}; observed {cache_controls!r}"
        )

    allowed = {expected_output_root, DISABLED_REPO_CONTENTS_CACHE}
    unreviewed = [option for option in startup if option not in allowed]
    if unreviewed:
        raise ValueError(
            "credential-free Windows gnullvm exact startup vector rejects unreviewed "
            f"base startup options: {unreviewed!r}"
        )

    return [expected_output_root, DISABLED_REPO_CONTENTS_CACHE]
