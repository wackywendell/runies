# Runies

Prints information about unicode data in a string. Separates grapheme clusters and characters, prints lengths, and prints character codepoint, bytes, category, and name.

## Usage

Using `cargo run`:

```
$ cargo run < test_text.txt
Input:      👋🏼 Hello! 👩🏾‍❤️‍💋‍👩🏻
Length:     10 grapheme clusters, 20 chars, 51 bytes
Clusters: 
  "👋🏼" : 8 bytes, 2 chars
    "👋" : 0x1f44b (128075), bytes [f0, 9f, 91, 8b], category: So, block: Miscellaneous Symbols and Pictographs, name: WAVING HAND SIGN
    "🏼" : 0x1f3fc (127996), bytes [f0, 9f, 8f, bc], category: Sk, block: Miscellaneous Symbols and Pictographs, name: EMOJI MODIFIER FITZPATRICK TYPE-3
  " " : 0x20 (32), bytes [20], category: Zs, block: Basic Latin, name: SPACE
  "H" : 0x48 (72), bytes [48], category: Lu, block: Basic Latin, name: LATIN CAPITAL LETTER H
  "e" : 0x65 (101), bytes [65], category: Ll, block: Basic Latin, name: LATIN SMALL LETTER E
  "l" : 0x6c (108), bytes [6c], category: Ll, block: Basic Latin, name: LATIN SMALL LETTER L
  "l" : 0x6c (108), bytes [6c], category: Ll, block: Basic Latin, name: LATIN SMALL LETTER L
  "o" : 0x6f (111), bytes [6f], category: Ll, block: Basic Latin, name: LATIN SMALL LETTER O
  "!" : 0x21 (33), bytes [21], category: Po, block: Basic Latin, name: EXCLAMATION MARK
  " " : 0x20 (32), bytes [20], category: Zs, block: Basic Latin, name: SPACE
  "👩🏾‍❤️‍💋‍👩🏻" : 35 bytes, 10 chars
    "👩" : 0x1f469 (128105), bytes [f0, 9f, 91, a9], category: So, block: Miscellaneous Symbols and Pictographs, name: WOMAN
    "🏾" : 0x1f3fe (127998), bytes [f0, 9f, 8f, be], category: Sk, block: Miscellaneous Symbols and Pictographs, name: EMOJI MODIFIER FITZPATRICK TYPE-5
    "‍" : 0x200d (8205), bytes [e2, 80, 8d], category: Cf, block: General Punctuation, name: ZERO WIDTH JOINER
    "❤" : 0x2764 (10084), bytes [e2, 9d, a4], category: So, block: Dingbats, name: HEAVY BLACK HEART
    "️" : 0xfe0f (65039), bytes [ef, b8, 8f], category: Mn, block: Variation Selectors, name: VARIATION SELECTOR-16
    "‍" : 0x200d (8205), bytes [e2, 80, 8d], category: Cf, block: General Punctuation, name: ZERO WIDTH JOINER
    "💋" : 0x1f48b (128139), bytes [f0, 9f, 92, 8b], category: So, block: Miscellaneous Symbols and Pictographs, name: KISS MARK
    "‍" : 0x200d (8205), bytes [e2, 80, 8d], category: Cf, block: General Punctuation, name: ZERO WIDTH JOINER
    "👩" : 0x1f469 (128105), bytes [f0, 9f, 91, a9], category: So, block: Miscellaneous Symbols and Pictographs, name: WOMAN
    "🏻" : 0x1f3fb (127995), bytes [f0, 9f, 8f, bb], category: Sk, block: Miscellaneous Symbols and Pictographs, name: EMOJI MODIFIER FITZPATRICK TYPE-1-2

```