# Copyright 2022 Jeffrey M Hodges.
# SPDX-License-Identifier: Apache-2.0

from unittest import TestCase

import biscuit_auth
from biscuit_auth import Biscuit, KeyPair


class TestBiscuitAuth(TestCase):
    def test_golden_path(self):
        self.assertTrue(biscuit_auth.hello())

    def test_new_keypair_builder_add_fact_only_pred_name_default_symbols(self):

        pair = KeyPair()
        builder = Biscuit.builder(pair)
        builder.add_authority_fact_only_predicate_name("read")
        biscuit = builder.build()
        self.assertTrue(biscuit)
