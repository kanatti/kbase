; Extract all ATX-style headings (# through ######)
;
; Pattern Structure:
;   Each pattern creates ONE match per heading with TWO captures:
;   - @h1/@h2/etc captures the entire atx_heading node (determines level)
;   - @text captures the inline content (the actual heading text)
;   Both captures belong to the same match, guaranteed to correspond.
;
; Example: "# Hello World" produces:
;   Match { captures: [@h1 → <entire heading>, @text → "Hello World"] }
;
; Note: Only matches headings with content. Empty headings like `#` are ignored
; because the (inline) child must be present for the pattern to match.

(atx_heading
  (atx_h1_marker)
  (inline) @text) @h1

(atx_heading
  (atx_h2_marker)
  (inline) @text) @h2

(atx_heading
  (atx_h3_marker)
  (inline) @text) @h3

(atx_heading
  (atx_h4_marker)
  (inline) @text) @h4

(atx_heading
  (atx_h5_marker)
  (inline) @text) @h5

(atx_heading
  (atx_h6_marker)
  (inline) @text) @h6
