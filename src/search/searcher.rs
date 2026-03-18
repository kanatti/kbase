use crate::search::domain::{SearchQuery, SearchResult, SearchResults, Relevance};
use crate::search::schema::SearchSchema;
use crate::search::repository::SearchRepository;
use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::{Query, BooleanQuery, TermQuery, Occur};
use tantivy::{Term, TantivyDocument, schema::IndexRecordOption};
use tantivy::schema::Value;

/// Service for executing search queries
pub struct SearchService {
    repository: SearchRepository,
    schema: SearchSchema,
}

impl SearchService {
    pub fn new(repository: SearchRepository) -> Self {
        SearchService {
            repository,
            schema: SearchSchema::new(),
        }
    }
    
    /// Execute a search query
    pub fn search(&self, query: &SearchQuery) -> Result<SearchResults> {
        // Open index
        let index = self.repository.open()?;
        let reader = index.reader()?;
        let searcher = reader.searcher();
        
        // Build Tantivy query
        let tantivy_query = self.build_query(&index, query)?;
        
        // Execute search
        let top_docs = searcher.search(
            &tantivy_query,
            &TopDocs::with_limit(query.limit())
        )?;
        
        // Transform results to domain entities
        let results = self.extract_results(&searcher, top_docs)?;
        
        Ok(SearchResults::new(results, query.term().to_string()))
    }
    
    fn build_query(&self, index: &tantivy::Index, query: &SearchQuery) -> Result<Box<dyn Query>> {
        let parser = self.schema.create_query_parser(index);
        
        // Parse text query
        let text_query = parser.parse_query(query.term())?;
        
        // Apply domain filter if present
        if let Some(domain) = query.domain_filter() {
            Ok(Box::new(BooleanQuery::new(vec![
                (Occur::Must, text_query),
                (Occur::Must, self.create_domain_filter(domain)),
            ])))
        } else {
            Ok(text_query)
        }
    }
    
    fn create_domain_filter(&self, domain: &str) -> Box<dyn Query> {
        let term = Term::from_field_text(self.schema.fields().domain, domain);
        Box::new(TermQuery::new(term, IndexRecordOption::Basic))
    }
    
    fn extract_results(
        &self,
        searcher: &tantivy::Searcher,
        top_docs: Vec<(f32, tantivy::DocAddress)>
    ) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        
        for (score, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            let result = self.doc_to_result(doc, score)?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    fn doc_to_result(&self, doc: TantivyDocument, score: f32) -> Result<SearchResult> {
        let fields = self.schema.fields();
        
        let path = self.extract_field_text(&doc, fields.path)?.to_string();
        let domain = self.extract_field_text(&doc, fields.domain)?.to_string();
        let title = self.extract_field_text(&doc, fields.title)?.to_string();
        let relevance = Relevance::new(score)?;
        
        Ok(SearchResult::new(path, title, domain, relevance))
    }
    
    fn extract_field_text<'a>(&self, doc: &'a TantivyDocument, field: tantivy::schema::Field) -> Result<&'a str> {
        doc.get_first(field)
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing field in document"))
    }
}
