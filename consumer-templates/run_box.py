"""
Runs a local box through the typed Python consumer.

The Python consumer is published separately on PyPI.
npm install scrollcase does not install this Python package.

SETUP (once):

    python -m pip install scrollcase-consumer

RUN (from the project root):

    python consumer-templates/run_box.py

Replace <target> and <hash> below with the values printed by scrollcase build.
"""

from __future__ import annotations

import sys

from scrollcase_consumer import PreparedBox, run_box


RELEASE_TO_RUN = (
    ".scrollcase/dist/boxes/example-box/1.0.0/<target>/<hash>.release.json"
)


def _report(prepared: PreparedBox) -> None:
    print(
        f"Running {prepared.box_id} {prepared.version} ({prepared.target_id})"
    )


def main() -> int:
    result = run_box(
        RELEASE_TO_RUN,
        public_key_path=".scrollcase/keys/signing-public.json",
        args=[],
        on_prepared=_report,
    )

    if result.signal is not None:
        print(f"Box exited after {result.signal}.", file=sys.stderr)
    return result.exit_code if result.exit_code is not None else 1


if __name__ == "__main__":
    raise SystemExit(main())
