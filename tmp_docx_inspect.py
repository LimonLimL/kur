# -*- coding: utf-8 -*-
"""Временный скрипт: осмотр структуры документа."""
import docx

PATH = r"D:\Курсовая (Мурзин Дмитрий)\Курсовая_РГСУ_Мурзин_АИ-203_2.docx"

doc = docx.Document(PATH)

print("PARAGRAPH STYLES:")
for s in doc.styles:
    if str(s.type) == "PARAGRAPH (1)":
        print("  ", s.name)
print("TABLE STYLES:")
for s in doc.styles:
    if str(s.type) == "TABLE (3)":
        print("  ", s.name)

print("TOTAL PARAGRAPHS:", len(doc.paragraphs))
print("TOTAL TABLES:", len(doc.tables))

print("LAST 15 PARAGRAPHS:")
for p in doc.paragraphs[-15:]:
    print("   [", p.style.name, "]", repr(p.text[:80]))

print("HEADING STYLES USED:")
used = {}
for p in doc.paragraphs:
    used.setdefault(p.style.name, 0)
    used[p.style.name] += 1
for k, v in used.items():
    print("   ", k, v)
