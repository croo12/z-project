import { Interaction, Feedback } from "../types";
import { randomUUID } from "crypto";
import logger from "../lib/logger";

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
  }
}

export const feedbackService = new FeedbackService();
