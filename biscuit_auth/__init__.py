# Copyright 2022 Jeffrey M Hodges.
# SPDX-License-Identifier: Apache-2.0

from .biscuit_auth import KeyPair

__all__ = [
    "hello",
]


def hello():
    return KeyPair()
