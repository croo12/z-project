export type AIProvider = 'openai' | 'gemini';

export interface AIModelConfig {
  /**
   * 사용할 AI 공급자
   */
  provider: AIProvider;

  /**
   * API 키 (생략 시 환경 변수 사용)
   * OpenAI: OPENAI_API_KEY
   * Gemini: GEMINI_API_KEY
   */
  apiKey?: string;

  /**
   * 사용할 모델명 (예: 'gpt-4o', 'gemini-1.5-pro')
   * 생략 시 공급자별 기본값 사용
   */
  modelName?: string;

  /**
   * 생성 온도 (0.0 ~ 1.0)
   * default: 0.7
   */
  temperature?: number;

  /**
   * 최대 생성 토큰 수
   */
  maxTokens?: number;

  /**
   * 스트리밍 활성화 여부
   * default: false
   */
  streaming?: boolean;

  /**
   * 추가 공급자별 옵션
   */
  additionalKwargs?: Record<string, any>;
}
