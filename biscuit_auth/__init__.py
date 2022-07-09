# Copyright 2022 Jeffrey M Hodges.
# SPDX-License-Identifier: Apache-2.0

from .biscuit_auth import __all__ as rust_all, KeyPair

__all__ = rust_all + [
    "hello",
]


def hello():
    return KeyPair()
