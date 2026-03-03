# Rusty-RAG Documentation

## Overview

`rusty-rag` is a high-performance, local-first vector search engine written in Rust and compiled to WebAssembly (WASM). Designed for seamless integration into serverless architectures like Next.js, it shifts embedding storage and similarity calculations from the server directly to the client's browser. It enables zero-latency Retrieval-Augmented Generation (RAG) by combining hardware-accelerated math, advanced graph search algorithms, and persistent local storage.

## Advantages

* **Zero Network Latency:** Similarity searches execute entirely on the client side, bypassing server round-trips.
* **Cost Efficiency:** Offloads computationally expensive HNSW and KNN operations from your serverless functions to the user's device.
* **Privacy-Preserving:** User embeddings and local search graphs never leave the browser.
* **High Density Storage:** Stores ~100k 1536-dimensional vectors in just ~120-160 MB of client storage.

## Workflow Integration

1. **Initialization:** The server delivers the WASM module to the client. The client creates a singular, optimized IndexedDB instance to manage the vector state.
2. **Add/Store:** The server pushes the relevant search space (item-id:vector pairs) to the browser. The module compresses and stores these in IndexedDB.
3. **Search:** A user submits a text query. The client converts the text to a vector via an API, then passes the vector to `rusty-rag`. The module queries its local index and returns the Top-K nearest neighbors.
4. **Delete/Update:** As the user's relevant search space changes, the server instructs the client to delete (tombstone) stale vectors and add new ones.
5. **Crash Recovery:** If the browser clears IndexedDB or a critical error occurs, the module detects the missing state and pings the server to re-hydrate the database.

## Technical In-Depth Analysis

### WASM & SIMD128

`rusty-rag` is compiled with the `simd128` target feature. This allows the WASM runtime to utilize Single Instruction, Multiple Data (SIMD) CPU registers, processing multiple vector dimensions simultaneously. This hardware-level parallelism is critical for achieving sub-millisecond latency when calculating Cosine or L2 distances across thousands of vectors.

### Scalar Quantization (SQ8)

To bypass browser storage limits, vectors undergo INT8 scalar quantization before hitting IndexedDB. A standard 1536-dimensional vector (using 32-bit floats) takes ~6.1 KB. Quantizing down to 8-bit integers compresses this to ~1.5 KB per vector, allowing massive datasets to reside locally with a negligible drop in recall accuracy.

### HNSW & LRU Cache Bridge

The core search algorithm is Hierarchical Navigable Small World (HNSW), which provides logarithmic time complexity for nearest neighbor searches. Because crossing the JS-WASM boundary to fetch nodes from IndexedDB during graph traversal is highly inefficient, `rusty-rag` employs an internal LRU cache inside the WASM linear memory. This cache holds the most frequently accessed upper-layer graph nodes, drastically reducing I/O bottlenecks.

### Deletion and Soft-Tombstoning

Standard HNSW graphs do not support easy deletions without breaking network connectivity. `rusty-rag` implements soft deletes (tombstoning). Deleted IDs are marked as invalid in the bitset and ignored during search results, while remaining in the graph to maintain structural integrity. A background routine can eventually compact the graph when tombstone density gets too high.

## Example Integration

```javascript
// havenot thought now leave it as it is
import { RustyRag } from 'rusty-rag';

async function initSearch() {
  const rag = await RustyRag.init({
    dimension: 1536,
    metric: 'cosine',
    useSimd: true
  });

  rag.onCrash(() => {
    console.warn("IndexedDB wiped. Requesting state sync from server...");
    requestServerSync();
  });

  return rag;
}

```
