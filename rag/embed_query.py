#!/usr/bin/env python3
"""
Simple script to generate embeddings for a query text.
Used by Rust to get embeddings for search queries.
"""
import sys
import json
from sentence_transformers import SentenceTransformer

def main():
    if len(sys.argv) != 2:
        print("Usage: python embed_query.py 'query text'", file=sys.stderr)
        sys.exit(1)
    
    query_text = sys.argv[1]
    
    # Load the same model used for embedding documents
    model = SentenceTransformer('all-MiniLM-L6-v2')
    
    # Generate embedding
    embedding = model.encode(query_text)
    
    # Output as JSON array
    print(json.dumps(embedding.tolist()))

if __name__ == "__main__":
    main()