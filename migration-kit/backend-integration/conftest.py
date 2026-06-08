"""Conftest raiz do backend-integration.

Adiciona o diretorio backend-integration/ ao sys.path para que os
testes sob tests/ possam fazer `from services.X import ...` sem
precisar de pyproject/pytest.ini.

Este conftest vive na raiz (e nao em tests/conftest.py) porque
pytest, ao encontrar __init__.py em tests/, entra em "package
mode" e nao executa conftest abaixo do package root. Em modo
"rootdir" (sem __init__.py), o rootdir vira backend-integration/
e este conftest e' executado normalmente.
"""
from __future__ import annotations

import sys
from pathlib import Path

_BACKEND_ROOT = Path(__file__).resolve().parent
if str(_BACKEND_ROOT) not in sys.path:
    sys.path.insert(0, str(_BACKEND_ROOT))
