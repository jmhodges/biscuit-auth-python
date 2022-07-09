#!/bin/bash

# Copyright 2022 Jeffrey M Hodges.
# SPDX-License-Identifier: Apache-2.0

maturin develop && python -m unittest discover -s biscuit_auth/tests
