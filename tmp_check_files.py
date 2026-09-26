# -*- coding: utf-8 -*-
"""Временный скрипт: проверка файлов в папке проекта."""
import os
import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

folder = r"D:\Курсовая (Мурзин Дмитрий)"
for name in sorted(os.listdir(folder)):
    full = os.path.join(folder, name)
    print(repr(name), os.path.getsize(full) if os.path.isfile(full) else "<DIR>")

target = os.path.join(folder, "Курсовая_РГСУ_Мурзин_АИ-203_2.docx")
print("target exists:", os.path.exists(target))
try:
    with open(target, "rb") as fh:
        print("readable, first bytes:", fh.read(4))
except OSError as exc:
    print("OPEN ERROR:", type(exc).__name__, exc)
