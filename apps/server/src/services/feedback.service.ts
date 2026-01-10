import { Interaction, Feedback } from "../types/index.js";
import { randomUUID } from "crypto";
import logger from "../lib/logger.js";
import { vectorStoreService } from "../lib/vector-store.js";

// In-memory store for MVP. Replace with a proper database later.
const interactions: Map<string, Interaction> = new Map();
const feedbacks: Map<string, Feedback> = new Map();

export class FeedbackService {
  async storeInteraction(interactionData: Omit<Interaction, 'id' | 'timestamp'>): Promise<string> {
    const id = randomUUID();
    const interaction: Interaction = {
      id,
      timestamp: new Date(),
      ...interactionData,
    };
    interactions.set(id, interaction);
    logger.info({ interactionId: id }, "Stored interaction.");
    logger.debug({ interactions }, "Current interactions map:");
    return id;
  }

  async storeFeedback(feedbackData: Omit<Feedback, 'id' | 'timestamp'>): Promise<void> {
    if (!interactions.has(feedbackData.interaction_id)) {
      throw new Error(`Interaction with id ${feedbackData.interaction_id} not found.`);
    }
    const id = randomUUID();
    const feedback: Feedback = {
      id,
      timestamp: new Date(),
      ...feedbackData,
    };
    feedbacks.set(id, feedback);
    logger.info({ feedbackId: id, interactionId: feedback.interaction_id }, "Stored feedback.");
    logger.debug({ feedbacks }, "Current feedbacks map:");

    // Adjust scores based on feedback
    const interaction = interactions.get(feedbackData.interaction_id);
    if (interaction && interaction.source_documents) {
      for (const docMetadata of interaction.source_documents) {
        if (docMetadata.id) {
          const currentScore = docMetadata.retrieval_score_modifier || 1.0;
          let newScore = currentScore;

          if (feedback.rating === 'positive') {
            newScore += 0.1;
          } else if (feedback.rating === 'negative') {
            // decay faster? or symmetric?
            newScore -= 0.1;
          }

          if (newScore !== currentScore) {
             await vectorStoreService.updateScore(docMetadata.id, newScore);
          }
        }
      }
    }
  }
}

export const feedbackService = new FeedbackService();
