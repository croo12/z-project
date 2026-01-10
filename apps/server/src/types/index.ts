export interface Interaction {
  id: string; // UUID
  user_context?: Record<string, any>;
  original_query: string;
  ai_response: string;
  source_documents?: Record<string, any>[];
  timestamp: Date;
}

export interface Feedback {
  id: string; // UUID
  interaction_id: string; // FK to Interaction
  rating: 'positive' | 'negative';
  correction?: string;
  timestamp: Date;
}

export interface KnowledgeChunk {
  id: string;
  content: string;
  metadata: {
    source: string;
    ingested_at: Date;
    retrieval_score_modifier?: number;
    [key: string]: any;
  };
}
