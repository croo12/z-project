import { Router } from 'express';
import { feedbackService } from '../services/feedback.service.js';

const router = Router();

router.post('/', async (req, res, next) => {
  try {
    const { user_context, original_query, ai_response, source_documents } = req.body;
    if (!original_query || !ai_response) {
      return res.status(400).json({ error: 'Missing required fields: original_query and ai_response' });
    }

    const interactionId = await feedbackService.storeInteraction({
      user_context,
      original_query,
      ai_response,
      source_documents,
    });

    res.status(201).json({ id: interactionId });
  } catch (error) {
    next(error);
  }
});

export default router;
