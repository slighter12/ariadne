import React, { useState } from 'react';
import type { RelationFormData, TimeRange } from '../types';
import { TimeSelector } from './TimeSelector';
import { getVideoIdFromUrl } from '../utils/youtube';

interface RelationFormProps {
  onSubmit: (data: RelationFormData) => void;
  isLoading?: boolean;
  className?: string;
}

export const RelationForm: React.FC<RelationFormProps> = ({
  onSubmit,
  isLoading = false,
  className = ''
}) => {
  const [formData, setFormData] = useState<RelationFormData>({
    sourceVideoUrl: '',
    sourceStartTime: 0,
    sourceEndTime: 30,
    targetVideoUrl: '',
    targetStartTime: 0,
    targetEndTime: 30
  });

  const [errors, setErrors] = useState<Partial<RelationFormData>>({});

  const validateForm = (): boolean => {
    const newErrors: Partial<RelationFormData> = {};

    // 驗證來源影片 URL
    if (!formData.sourceVideoUrl) {
      newErrors.sourceVideoUrl = '請輸入來源影片 URL';
    } else if (!getVideoIdFromUrl(formData.sourceVideoUrl)) {
      newErrors.sourceVideoUrl = '請輸入有效的 YouTube 影片 URL';
    }

    // 驗證目標影片 URL
    if (!formData.targetVideoUrl) {
      newErrors.targetVideoUrl = '請輸入目標影片 URL';
    } else if (!getVideoIdFromUrl(formData.targetVideoUrl)) {
      newErrors.targetVideoUrl = '請輸入有效的 YouTube 影片 URL';
    }

    // 驗證時間範圍
    if (formData.sourceStartTime >= formData.sourceEndTime) {
      newErrors.sourceStartTime = '開始時間必須小於結束時間' as any;
    }

    if (formData.targetStartTime >= formData.targetEndTime) {
      newErrors.targetStartTime = '開始時間必須小於結束時間' as any;
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    if (validateForm()) {
      onSubmit(formData);
    }
  };

  const handleSourceTimeChange = (range: TimeRange) => {
    setFormData(prev => ({
      ...prev,
      sourceStartTime: range.start,
      sourceEndTime: range.end
    }));
  };

  const handleTargetTimeChange = (range: TimeRange) => {
    setFormData(prev => ({
      ...prev,
      targetStartTime: range.start,
      targetEndTime: range.end
    }));
  };

  const handleCurrentVideoAsTarget = () => {
    setFormData(prev => ({
      ...prev,
      targetVideoUrl: window.location.href
    }));
  };

  return (
    <form onSubmit={handleSubmit} className={`space-y-6 ${className}`}>
      <div className="space-y-4">
        {/* 來源影片 */}
        <div>
          <h3 className="text-lg font-medium text-gray-900 mb-3">來源影片</h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                影片 URL
              </label>
              <input
                type="url"
                value={formData.sourceVideoUrl}
                onChange={(e) => setFormData(prev => ({ ...prev, sourceVideoUrl: e.target.value }))}
                placeholder="https://www.youtube.com/watch?v=..."
                className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                  errors.sourceVideoUrl ? 'border-red-500' : 'border-gray-300'
                }`}
              />
              {errors.sourceVideoUrl && (
                <p className="mt-1 text-sm text-red-600">{errors.sourceVideoUrl}</p>
              )}
            </div>

            <TimeSelector
              value={{ start: formData.sourceStartTime, end: formData.sourceEndTime }}
              onChange={handleSourceTimeChange}
              maxDuration={3600} // 預設最大時長 1 小時
              className="border border-gray-200 rounded-lg p-4"
            />
          </div>
        </div>

        {/* 目標影片 */}
        <div>
          <h3 className="text-lg font-medium text-gray-900 mb-3">目標影片</h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                影片 URL
              </label>
              <div className="flex space-x-2">
                <input
                  type="url"
                  value={formData.targetVideoUrl}
                  onChange={(e) => setFormData(prev => ({ ...prev, targetVideoUrl: e.target.value }))}
                  placeholder="https://www.youtube.com/watch?v=..."
                  className={`flex-1 px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                    errors.targetVideoUrl ? 'border-red-500' : 'border-gray-300'
                  }`}
                />
                <button
                  type="button"
                  onClick={handleCurrentVideoAsTarget}
                  className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors"
                >
                  當前影片
                </button>
              </div>
              {errors.targetVideoUrl && (
                <p className="mt-1 text-sm text-red-600">{errors.targetVideoUrl}</p>
              )}
            </div>

            <TimeSelector
              value={{ start: formData.targetStartTime, end: formData.targetEndTime }}
              onChange={handleTargetTimeChange}
              maxDuration={3600} // 預設最大時長 1 小時
              className="border border-gray-200 rounded-lg p-4"
            />
          </div>
        </div>
      </div>

      {/* 提交按鈕 */}
      <div className="flex justify-end space-x-3">
        <button
          type="button"
          onClick={() => setFormData({
            sourceVideoUrl: '',
            sourceStartTime: 0,
            sourceEndTime: 30,
            targetVideoUrl: '',
            targetStartTime: 0,
            targetEndTime: 30
          })}
          className="px-4 py-2 text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 transition-colors"
        >
          重置
        </button>
        <button
          type="submit"
          disabled={isLoading}
          className="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {isLoading ? '提交中...' : '提交關聯'}
        </button>
      </div>
    </form>
  );
}; 