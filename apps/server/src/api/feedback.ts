import { Router } from 'express';
import { feedbackService } from '../services/feedback.service.js';

const router = Router();

router.post('/', async (req, res, next) => {
  try {
    const { interaction_id, rating, correction } = req.body;
    if (!interaction_id || !rating) {
      return res.status(400).json({ error: 'Missing required fields: interaction_id and rating' });
    }

    await feedbackService.storeFeedback({
      interaction_id,
      rating,
      correction,
    });

    res.status(201).json({ message: 'Feedback submitted.' });
  } catch (error) {
    next(error);
  }
});

export default router;
