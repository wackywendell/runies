# Runies

Prints information about unicode in a string.

## Usage

Using `cargo run`:

```
$ cargo run < test_text.txt
Input:      🙂 Hello! 👩🏾‍❤️‍💋‍👩🏻
Length:     47 bytes, 19 chars
Characters: 
  "🙂" : 0x1f642 (128578), bytes [f0, 9f, 99, 82], name: SLIGHTLY SMILING FACE
  " " : 0x20 (32), bytes [20], name: SPACE
  "H" : 0x48 (72), bytes [48], name: LATIN CAPITAL LETTER H
  "e" : 0x65 (101), bytes [65], name: LATIN SMALL LETTER E
  "l" : 0x6c (108), bytes [6c], name: LATIN SMALL LETTER L
  "l" : 0x6c (108), bytes [6c], name: LATIN SMALL LETTER L
  "o" : 0x6f (111), bytes [6f], name: LATIN SMALL LETTER O
  "!" : 0x21 (33), bytes [21], name: EXCLAMATION MARK
  " " : 0x20 (32), bytes [20], name: SPACE
  "👩" : 0x1f469 (128105), bytes [f0, 9f, 91, a9], name: WOMAN
  "🏾" : 0x1f3fe (127998), bytes [f0, 9f, 8f, be], name: EMOJI MODIFIER FITZPATRICK TYPE-5
  "‍" : 0x200d (8205), bytes [e2, 80, 8d], name: ZERO WIDTH JOINER
  "❤" : 0x2764 (10084), bytes [e2, 9d, a4], name: HEAVY BLACK HEART
  "️" : 0xfe0f (65039), bytes [ef, b8, 8f], name: VARIATION SELECTOR-16
  "‍" : 0x200d (8205), bytes [e2, 80, 8d], name: ZERO WIDTH JOINER
  "💋" : 0x1f48b (128139), bytes [f0, 9f, 92, 8b], name: KISS MARK
  "‍" : 0x200d (8205), bytes [e2, 80, 8d], name: ZERO WIDTH JOINER
  "👩" : 0x1f469 (128105), bytes [f0, 9f, 91, a9], name: WOMAN
  "🏻" : 0x1f3fb (127995), bytes [f0, 9f, 8f, bb], name: EMOJI MODIFIER FITZPATRICK TYPE-1-2
```