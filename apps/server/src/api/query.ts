import { Router } from 'express';
import { ragGraph, AgentState } from '../core/graph.js';
import { Document } from "@langchain/core/documents";

const router = Router();

router.post('/', async (req, res, next) => {
  try {
    const { query, context } = req.body;
    if (!query) {
      return res.status(400).json({ error: 'Missing required field: query' });
    }

    const finalState = (await ragGraph.invoke({
      query,
      context,
    })) as unknown as AgentState;

    res.status(200).json({ 
      response: finalState.response,
      source_documents: finalState.documents.map((doc: Document) => doc.metadata)
    });
  } catch (error) {
    next(error);
  }
});

export default router;
