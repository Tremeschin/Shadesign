# /// script
# dependencies = [
#     "wordfreq[cjk]==3.1.1",
# ]
# ///

# pyright: reportMissingImports=false

import gzip
import itertools
from pathlib import Path

import wordfreq

# Fun languages to work, later words are less useful (duped, out of context)
for language in ("de el en es fr it ja ko nl pl pt ru zh").split():
    with gzip.open(Path(__file__).parent.joinpath(f"{language}.txt.gz"), "wb") as file:
        words = list(itertools.islice(wordfreq.iter_wordlist(language), 50000))
        file.write("\n".join(words).encode("utf-8"))
