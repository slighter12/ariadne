import type { Relation, RelationDisplay } from '../types';

// 將後端 Relation 轉換為前端顯示格式
export function transformRelationToDisplay(relation: Relation): RelationDisplay {
  return {
    id: relation.id,
    sourceVideoId: relation.source_video_id,
    sourceVideoUrl: `https://www.youtube.com/watch?v=${relation.source_video_id}`,
    sourceStartTime: relation.source_start_time,
    sourceEndTime: relation.source_end_time,
    targetVideoId: relation.target_video_id,
    targetVideoUrl: `https://www.youtube.com/watch?v=${relation.target_video_id}`,
    targetStartTime: relation.target_start_time,
    targetEndTime: relation.target_end_time,
    status: relation.status,
    createdAt: relation.created_at,
    createdBy: relation.user_id,
  };
}

// 將前端表單資料轉換為後端格式
export function transformFormDataToBackend(formData: {
  sourceVideoUrl: string;
  sourceStartTime: number;
  sourceEndTime: number;
  targetVideoUrl: string;
  targetStartTime: number;
  targetEndTime: number;
}) {
  const getVideoIdFromUrl = (url: string): string | null => {
    try {
      const urlObj = new URL(url);
      if (urlObj.hostname.includes('youtube.com') || urlObj.hostname.includes('youtu.be')) {
        if (urlObj.hostname.includes('youtu.be')) {
          return urlObj.pathname.slice(1);
        } else {
          return urlObj.searchParams.get('v');
        }
      }
    } catch {
      // Invalid URL
    }
    return null;
  };

  const sourceVideoId = getVideoIdFromUrl(formData.sourceVideoUrl);
  const targetVideoId = getVideoIdFromUrl(formData.targetVideoUrl);

  if (!sourceVideoId || !targetVideoId) {
    throw new Error('Invalid YouTube URL');
  }

  return {
    source_video_id: sourceVideoId,
    source_start_time: formData.sourceStartTime,
    source_end_time: formData.sourceEndTime,
    target_video_id: targetVideoId,
    target_start_time: formData.targetStartTime,
    target_end_time: formData.targetEndTime,
    relation_type: 'reference' as const,
    user_id: null, // 暫時為 null，之後可以加入使用者系統
  };
}