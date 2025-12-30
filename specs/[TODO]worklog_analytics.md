# [TODO] 업무 로그 시각화 및 분석 (Work Log Analytics)

## 1. 개요 (Overview)
현재 Work Log 기능은 단순히 텍스트 리스트만 보여줍니다. 사용자가 자신의 시간 사용 패턴을 파악할 수 있도록 **차트와 통계** 기능을 제공합니다.

## 2. 목표 (Goals)
-   사용자가 프로젝트별 시간 투입 비중을 한눈에 파악할 수 있게 합니다.
-   일별/주별 업무 강도를 시각화합니다.

## 3. 요구사항 (Requirements)

### 3.1 통계 대시보드 (Dashboard Widget)
-   **위치**: DashboardView 상단 또는 별도의 'Stats' 탭.
-   **내용**:
    -   **오늘 총 근무 시간**: 텍스트로 크게 표시 (예: "Today: 6.5h").
    -   **이번 주 총 근무 시간**: 누적 시간 표시.

### 3.2 차트 시각화 (Charts)
1.  **프로젝트별 시간 분포 (Pie Chart / Donut Chart)**
    -   기간: 전체 또는 이번 주.
    -   데이터: 각 프로젝트명(`project`)별 `hours` 합계.
    -   라이브러리: `recharts`, `chart.js`, 또는 `visx`. (React 호환성 고려)

2.  **일별 근무 시간 추이 (Bar Chart)**
    -   X축: 날짜 (최근 7일).
    -   Y축: 근무 시간(h).

### 3.3 구현 상세
-   **Backend**:
    -   `get_work_log_stats(period: String)` Command 추가.
    -   DB에서 `GROUP BY project` 또는 `GROUP BY date` 쿼리를 수행하여 집계된 데이터를 반환하는 것이 효율적임.
-   **Frontend**:
    -   차트 라이브러리 설치 (`npm install recharts`).
    -   데이터 fetching 후 차트 컴포넌트에 전달.

## 4. 데이터 쿼리 예시 (SQL)

```sql
-- Project Distribution
SELECT project, SUM(hours) as total_hours
FROM work_logs
GROUP BY project;

-- Daily Trend
SELECT date, SUM(hours) as daily_total
FROM work_logs
WHERE date >= date('now', '-7 days')
GROUP BY date
ORDER BY date ASC;
```

## 5. 단계별 계획
1.  **라이브러리 선정**: 가볍고 React와 잘 맞는 차트 라이브러리 선정.
2.  **API 개발**: 집계 쿼리를 수행하는 Rust Command 작성.
3.  **UI 개발**: Dashboard에 차트 영역 배치 및 데이터 연동.
