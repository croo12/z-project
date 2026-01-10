import 'dotenv/config'; // Load environment variables first
import express from 'express';
import knowledgeRouter from './api/knowledge';
import interactionsRouter from './api/interactions';
import feedbackRouter from './api/feedback';
import queryRouter from './api/query';
import logger from './lib/logger';

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());

app.get('/', (req, res) => {
  res.send('AI Brain Server is running!');
});

app.use('/knowledge', knowledgeRouter);
app.use('/interactions', interactionsRouter);
app.use('/feedback', feedbackRouter);
app.use('/query', queryRouter);

app.listen(port, () => {
  logger.info(`Server is listening on port ${port}`);
});