import { Router } from 'express';
import { ragGraph } from '../core/graph';

const router = Router();

router.post('/', async (req, res, next) => {
  try {
    const { query, context } = req.body;
    if (!query) {
      return res.status(400).json({ error: 'Missing required field: query' });
    }

    const finalState = await ragGraph.invoke({
      query,
      context,
    });

    res.status(200).json({ 
      response: finalState.response,
      source_documents: finalState.documents.map(doc => doc.metadata)
    });
  } catch (error) {
    next(error);
  }
});

export default router;
