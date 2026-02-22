use anyhow::{Context, Result};
use tree_sitter::{Language, Query};

/// Compiled tree-sitter queries for extracting markdown elements
pub struct Queries {
    pub wikilinks: Query,
    pub tags: Query,
    pub headings: Query,
}

impl Queries {
    pub fn compile(block_lang: &Language, inline_lang: &Language) -> Result<Self> {
        let wikilinks = Query::new(inline_lang, include_str!("queries/wikilinks.scm"))
            .context("Failed to compile wikilink query")?;
        let tags = Query::new(inline_lang, include_str!("queries/tags.scm"))
            .context("Failed to compile tag query")?;
        let headings = Query::new(block_lang, include_str!("queries/headings.scm"))
            .context("Failed to compile heading query")?;

        Ok(Self {
            wikilinks,
            tags,
            headings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queries_compile() {
        let block_lang = &tree_sitter_md_obsidian::LANGUAGE.into();
        let inline_lang = &tree_sitter_md_obsidian::INLINE_LANGUAGE.into();

        let queries = Queries::compile(block_lang, inline_lang);
        assert!(queries.is_ok());
    }
}
