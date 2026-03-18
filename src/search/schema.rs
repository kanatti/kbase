use tantivy::schema::*;
use tantivy::{Index, query::QueryParser};

/// Tantivy schema wrapper with kbase-specific semantics
pub struct SearchSchema {
    schema: Schema,
    fields: SchemaFields,
}

impl SearchSchema {
    /// Create the standard kbase search schema
    pub fn new() -> Self {
        let mut builder = Schema::builder();
        
        let fields = SchemaFields {
            path: builder.add_text_field("path", STORED),
            domain: builder.add_text_field("domain", STRING | STORED),
            title: builder.add_text_field("title", TEXT | STORED),
            content: builder.add_text_field("content", TEXT),
        };
        
        SearchSchema {
            schema: builder.build(),
            fields,
        }
    }
    
    pub fn fields(&self) -> &SchemaFields { &self.fields }
    
    pub fn inner(&self) -> &Schema { &self.schema }
    
    /// Create a query parser configured for kbase queries
    /// 
    /// - Searches title and content fields
    /// - Title boost: 2.0×
    pub fn create_query_parser(&self, index: &Index) -> QueryParser {
        let mut parser = QueryParser::for_index(
            index,
            vec![self.fields.title, self.fields.content]
        );
        parser.set_field_boost(self.fields.title, 2.0);
        parser
    }
}

/// Field handles for the search schema
pub struct SchemaFields {
    pub path: Field,
    pub domain: Field,
    pub title: Field,
    pub content: Field,
}
