; Extract wikilinks from inline content
; Matches: [[target]] and [[target|alias]]

(wiki_link
  (link_destination) @target
  (link_text)? @alias) @link
