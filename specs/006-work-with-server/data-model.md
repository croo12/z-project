# 데이터 모델: 서버 본격 운용 및 아티클 입력 기능

**브랜치**: `006-work-with-server` | **날짜**: 2026-01-11

## 엔티티 정의

### Article (아티클)

| 필드 | 타입 | 필수 | 설명 |
|------|------|------|------|
| id | string (UUID) | ✅ | 고유 식별자 |
| title | string | ✅ | 아티클 제목 |
| url | string | ✅ | 원본 URL |
| content | string | ✅ | 본문 내용 (저장 시 청크 분할) |
| tags | string[] | ❌ | 태그 목록 |
| createdAt | ISO8601 Date | ✅ | 등록 일시 |
| updatedAt | ISO8601 Date | ✅ | 수정 일시 |
| chunkCount | number | ✅ | 벡터 DB에 저장된 청크 수 |
| rating | number | ✅ | 평균 평점 (기본값: 1.0, 범위: 0.0~2.0) |
| positiveCount | number | ✅ | 긍정적 피드백 수 (기본값: 0) |
| negativeCount | number | ✅ | 부정적 피드백 수 (기본값: 0) |

**검증 규칙**:
- `title`: 1~200자
- `url`: 유효한 URL 형식 (http:// 또는 https://)
- `content`: 최소 10자 이상
- `tags`: 각 태그는 1~50자, 최대 10개
- `rating`: 0.0 ~ 2.0 범위 (1.0이 기본값, 피드백에 따라 조정)

### ArticleMetadata (저장용 - JSON 파일)

벡터 DB에는 `content`가 청크로 분할되어 저장되므로, 메타데이터만 별도 파일에 저장합니다.

```typescript
interface ArticleMetadata {
  id: string;
  title: string;
  url: string;
  tags: string[];
  createdAt: string;
  updatedAt: string;
  chunkCount: number;
  rating: number;           // 평균 평점 (피드백 기반)
  positiveCount: number;    // 긍정적 피드백 수
  negativeCount: number;    // 부정적 피드백 수
}

interface ArticleStore {
  articles: ArticleMetadata[];
}
```

### ArticleFeedback (피드백)

| 필드 | 타입 | 필수 | 설명 |
|------|------|------|------|
| id | string (UUID) | ✅ | 피드백 고유 식별자 |
| articleId | string (UUID) | ✅ | 대상 아티클 ID |
| type | 'positive' \| 'negative' | ✅ | 피드백 유형 |
| comment | string | ❌ | 피드백 코멘트 |
| createdAt | ISO8601 Date | ✅ | 피드백 제출 일시 |

**점수 계산 공식**:
```
newRating = (positiveCount - negativeCount * 0.5) / totalFeedbackCount + 1.0
retrieval_score_modifier = rating (0.0 ~ 2.0 범위로 클램핑)
```

### VectorDocument (벡터 DB용 - LanceDB)

기존 `ingestion.service.ts`의 Document 구조를 확장합니다.

```typescript
interface DocumentMetadata {
  source: string;           // 출처 (아티클의 경우 URL)
  articleId?: string;       // 연결된 아티클 ID
  articleTitle?: string;    // 아티클 제목
  ingested_at: Date;
  retrieval_score_modifier: number;
}
```

## 관계도

```
Article (articles.json)
    │
    │ 1:N (articleId로 연결)
    ▼
VectorDocument (LanceDB)
    │
    │ N:1 (query에서 참조)
    ▼
RAGGraph (Query 처리)
```

## 상태 전이

### Article 상태

```
[생성 요청] 
    │
    ▼
[content 청크 분할]
    │
    ▼
[벡터 DB 저장]
    │
    ▼
[메타데이터 저장] ──▶ [등록 완료]
    │
    ▼
[삭제 요청] ──▶ [메타데이터 삭제] (벡터는 유지 - 향후 개선 필요)
```

## 인덱스 전략

### JSON 파일 (articles.json)
- 파일 크기가 작아 전체 로드 후 메모리 내 검색
- 향후 SQLite로 마이그레이션 시 `id`, `createdAt`에 인덱스 추가 고려

### LanceDB
- 기존 벡터 인덱스 활용
- `articleId` 메타데이터로 필터링 가능
