<!--
 Copyright 2022 Jeffrey M Hodges.
 SPDX-License-Identifier: Apache-2.0
-->


Build by creating a virtualenv and running `pip install`. Afterwards, you can
use the `maturin` command from the PyO3 project (e.g. `maturin develop`) and run
tests with `./test.sh`.

```
python3 -m venv .venv
source .venv/bin/activate
pip install .
```
