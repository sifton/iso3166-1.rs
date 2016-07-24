#!/usr/bin/python3
# ISC License (ISC)
#
# Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
#
# Permission to use, copy, modify, and/or distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.
#
# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
# SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
# RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
# CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
# CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
#
# What is ISO 3166-1?
#
# | ISO 3166-1 is part of the ISO 3166 standard published by the International
# | Organization for Standardization (ISO), and defines codes for the names of
# | countries, dependent territories, and special areas of geographical
# | interest.
# |
# | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)
#
# Originally by zeyla on GitHub.

# USAGE:
# Requires python3. Go to the following page, sort by the numeric range, copy
# the contents of the table minus the header cells, then run 'make update':
# https://en.wikipedia.org/wiki/ISO_3166-1#Officially_assigned_code_elements

import os
import re
import subprocess
import sys

# Get the contents from the clipboard.
clip = subprocess.run(['xsel', '--clipboard'], stdout=subprocess.PIPE)

# Split the clip into newlines, one item per index. This splits by 2+ spaces.
rows = clip.stdout.decode('utf-8').split("\n")

# Cycle through each row and append to the text its entry.
text = ""

for row in rows:
    # 0: Name
    # 1: Alpha2
    # 2: Alpha3
    # 3: Num
    cells = re.split(r'\s{2,}', row)

    text += '    codes.push(CountryCode {\n'
    text += '        alpha2: "{}",\n'.format(cells[1])
    text += '        alpha3: "{}",\n'.format(cells[2])
    text += '        name: "{}",\n'.format(cells[0])
    text += '        num: "{}",\n'.format(cells[3])
    text += '    });\n'

# Read the codes.rs file and split it to find the code after 'Begin' and before
# 'End'
codes_path = os.path.join(os.path.dirname(__file__), '../src/codes.rs')

with open(codes_path, 'r') as f:
    codes_file = f.read()

# Split by where to insert the text.
codes = codes_file.rsplit('// Begin', 1)
# And where to end putting the text.
codes_end = codes_file.rsplit('// End\n', 1)

with open(codes_path, 'w') as f:
    f.write(codes[0] + '// Begin\n' + text + '    // End\n' + codes_end[1])

print('Updated.')
