# Copyright 2022 Jeffrey M Hodges.
# SPDX-License-Identifier: Apache-2.0

import biscuit_auth
import unittest


from unittest import TestCase


class TestBiscuitAuth(TestCase):
    def test_golden_path(self):
        self.assertTrue(biscuit_auth.hello())
