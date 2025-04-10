# Theory

### UTF-8 -> binary
Characters are converted to UTF-8 bytes using standardized patterns:

| Bytes | Binary Pattern                        | Character Class |
|-------|---------------------------------------|-----------------|
| `1`   | `0xxxxxxx`                            | ASCII (Latin)   |
| `2`   | `110xxxxx 10xxxxxx`                   | Cyrillic        |
| `3`   | `1110xxxx 10xxxxxx 10xxxxxx`          | Special Symbols |
| `4`   | `11110xxx 10xxxxxx 10xxxxxx 10xxxxxx` | Emoji           |


**Key Rules:**
1. The leading byte determines character length by its prefix bits
2. All continuation bytes begin with `10xxxxxx`

**UTF-8 vs UTF-16**

- UTF-8: Variable-length (1-4 bytes), backward-compatible with ASCII.
- UTF-16: Fixed 2/4 bytes, no ASCII compatibility. (Windows/Java/legacy systems)

*Key Difference:* UTF-8 optimizes for space (ASCII-friendly), UTF-16 for fixed-width processing.