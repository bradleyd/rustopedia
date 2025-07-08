import os
import sys
import requests
import json
from pathlib import Path
import argparse
import hashlib
from sentence_transformers import SentenceTransformer

def chunk_text(text, chunk_size=800, overlap=100):
    """Split text into overlapping chunks"""
    words = text.split()
    chunks = []
    
    for i in range(0, len(words), chunk_size - overlap):
        chunk = ' '.join(words[i:i + chunk_size])
        if chunk.strip():
            chunks.append(chunk)
        
        # Stop if we've reached the end
        if i + chunk_size >= len(words):
            break
    
    return chunks

def extract_title(content):
    """Extract title from markdown content"""
    lines = content.split('\n')
    for line in lines:
        if line.startswith('# '):
            return line[2:].strip()
    return None

def create_chunk_id(file_path, chunk_index):
    """Create unique ID for chunk"""
    base = f"{file_path}#{chunk_index}"
    return hashlib.md5(base.encode()).hexdigest()

class QdrantClient:
    def __init__(self, url="http://localhost:6333"):
        self.url = url
        self.session = requests.Session()
    
    def create_collection(self, collection_name, vector_size=384):
        """Create collection with vector configuration"""
        payload = {
            "vectors": {
                "size": vector_size,
                "distance": "Cosine"
            }
        }
        response = self.session.put(
            f"{self.url}/collections/{collection_name}",
            json=payload
        )
        return response.status_code == 200
    
    def upsert_points(self, collection_name, points):
        """Insert/update points in collection"""
        payload = {"points": points}
        response = self.session.put(
            f"{self.url}/collections/{collection_name}/points",
            json=payload
        )
        return response.status_code == 200
    
    def collection_exists(self, collection_name):
        """Check if collection exists"""
        response = self.session.get(f"{self.url}/collections")
        if response.status_code == 200:
            collections = response.json()["result"]["collections"]
            return any(c["name"] == collection_name for c in collections)
        return False

def main():
    parser = argparse.ArgumentParser(description="Embed Markdown docs into Qdrant with chunking")
    parser.add_argument("--dir", required=True, help="Path to docs directory")
    parser.add_argument("--collection", required=True, help="Qdrant collection name")
    parser.add_argument("--chunk-size", type=int, default=800, help="Chunk size in words")
    parser.add_argument("--overlap", type=int, default=100, help="Overlap between chunks")
    parser.add_argument("--qdrant-url", default="http://localhost:6333", help="Qdrant server URL")
    args = parser.parse_args()

    # Initialize embedding model (same one ChromaDB uses by default)
    print("Loading embedding model...")
    model = SentenceTransformer('all-MiniLM-L6-v2')
    
    # Initialize Qdrant client
    qdrant = QdrantClient(args.qdrant_url)
    
    # Create collection if it doesn't exist
    if not qdrant.collection_exists(args.collection):
        print(f"Creating collection '{args.collection}'...")
        if not qdrant.create_collection(args.collection, vector_size=384):
            print(f"❌ Failed to create collection '{args.collection}'")
            sys.exit(1)
    else:
        print(f"Collection '{args.collection}' already exists")

    # Process documents
    docs = []
    points = []
    
    for file in Path(args.dir).rglob("*.md"):
        content = file.read_text(encoding="utf-8")
        title = extract_title(content)
        
        chunks = chunk_text(content, args.chunk_size, args.overlap)
        
        for i, chunk in enumerate(chunks):
            docs.append(chunk)
            point_id = create_chunk_id(str(file.resolve()), i)
            
            # Create point for Qdrant
            point = {
                "id": point_id,
                "payload": {
                    "text": chunk,
                    "file_path": str(file.resolve()),
                    "file_name": file.name,
                    "chunk_index": i,
                    "total_chunks": len(chunks),
                    "title": title or file.stem
                }
            }
            points.append(point)

    if not docs:
        print("❌ No .md files found in", args.dir)
        sys.exit(1)

    print(f"Generating embeddings for {len(docs)} chunks...")
    embeddings = model.encode(docs)
    
    # Add embeddings to points
    for point, embedding in zip(points, embeddings):
        point["vector"] = embedding.tolist()

    # Upload to Qdrant in batches
    batch_size = 100
    total_points = len(points)
    
    for i in range(0, total_points, batch_size):
        batch = points[i:i + batch_size]
        print(f"Uploading batch {i//batch_size + 1}/{(total_points + batch_size - 1)//batch_size}...")
        
        if not qdrant.upsert_points(args.collection, batch):
            print(f"❌ Failed to upload batch {i//batch_size + 1}")
            sys.exit(1)

    print(f"✅ Embedded {len(docs)} chunks from {len(set(p['payload']['file_path'] for p in points))} files into collection '{args.collection}'")

if __name__ == "__main__":
    main()