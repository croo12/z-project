import { Router } from 'express';
import { ingestionService } from '../services/ingestion.service';

const router = Router();

router.post('/', async (req, res, next) => {
  try {
    const { content, source } = req.body;
    if (!content || !source) {
      return res.status(400).json({ error: 'Missing required fields: content and source' });
    }

    await ingestionService.ingest(content, source);

    res.status(202).json({ message: 'Knowledge ingestion started.' });
  } catch (error) {
    next(error);
  }
});

export default router;
