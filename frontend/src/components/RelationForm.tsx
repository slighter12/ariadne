import React, { useState, useRef } from 'react';
import type { RelationFormData, TimeRange } from '../types';
import { TimeSelector } from './TimeSelector';
import { getVideoIdFromUrl } from '../utils/youtube';

interface RelationFormProps {
  onSubmit: (data: RelationFormData) => void;
  isLoading?: boolean;
  className?: string;
  currentVideoDuration?: number; // 當前影片的時長
}

export const RelationForm: React.FC<RelationFormProps> = ({
  onSubmit,
  isLoading = false,
  className = '',
  currentVideoDuration = 3600 // 預設最大時長 1 小時
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
  const sourceUrlRef = useRef<HTMLInputElement>(null);
  const targetUrlRef = useRef<HTMLInputElement>(null);

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

  // 將當前影片設為來源，並詢問是否貼上剪貼簿到目標
  const handleCurrentVideoAsSource = async () => {
    setFormData(prev => ({
      ...prev,
      sourceVideoUrl: window.location.href
    }));
    
    // 詢問是否要將剪貼簿內容貼到目標影片
    if (confirm('已將當前影片設為來源。是否要將剪貼簿中的 YouTube URL 貼到目標影片？')) {
      await handlePasteUrl('target');
    }
  };

  // 將當前影片設為目標，並詢問是否貼上剪貼簿到來源
  const handleCurrentVideoAsTarget = async () => {
    setFormData(prev => ({
      ...prev,
      targetVideoUrl: window.location.href
    }));
    
    // 詢問是否要將剪貼簿內容貼到來源影片
    if (confirm('已將當前影片設為目標。是否要將剪貼簿中的 YouTube URL 貼到來源影片？')) {
      await handlePasteUrl('source');
    }
  };

  // 從剪貼簿貼上 URL
  const handlePasteUrl = async (field: 'source' | 'target') => {
    try {
      const text = await navigator.clipboard.readText();
      if (text.includes('youtube.com/watch') || text.includes('youtu.be/')) {
        if (field === 'source') {
          setFormData(prev => ({ ...prev, sourceVideoUrl: text }));
        } else {
          setFormData(prev => ({ ...prev, targetVideoUrl: text }));
        }
      } else {
        alert('剪貼簿內容不是有效的 YouTube URL');
      }
    } catch (error) {
      console.error('無法讀取剪貼簿:', error);
      alert('無法讀取剪貼簿，請手動輸入 URL');
    }
  };

  // 處理鍵盤快捷鍵
  const handleKeyDown = (e: React.KeyboardEvent, field: 'source' | 'target') => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'v') {
      e.preventDefault();
      handlePasteUrl(field);
    }
  };

  return (
    <form onSubmit={handleSubmit} className={`space-y-6 ${className}`}>
      {/* 當前影片選擇 */}
      <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
        <h4 className="text-sm font-medium text-gray-900 mb-3">🎯 快速設置當前影片</h4>
        <div className="flex space-x-3">
          <button
            type="button"
            onClick={handleCurrentVideoAsSource}
            className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors text-sm font-medium"
          >
            設為來源影片
          </button>
          <button
            type="button"
            onClick={handleCurrentVideoAsTarget}
            className="flex-1 px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors text-sm font-medium"
          >
            設為目標影片
          </button>
        </div>
        <p className="text-xs text-gray-600 mt-2">
          選擇後會詢問是否要將剪貼簿中的 YouTube URL 貼到另一邊
        </p>
      </div>

      <div className="space-y-4">
        {/* 來源影片 */}
        <div>
          <h3 className="text-lg font-medium text-gray-900 mb-3">來源影片</h3>
          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                影片 URL
              </label>
              <div className="flex space-x-2">
                <input
                  ref={sourceUrlRef}
                  type="url"
                  value={formData.sourceVideoUrl}
                  onChange={(e) => setFormData(prev => ({ ...prev, sourceVideoUrl: e.target.value }))}
                  onKeyDown={(e) => handleKeyDown(e, 'source')}
                  placeholder="https://www.youtube.com/watch?v=... (Ctrl+V 貼上)"
                  className={`flex-1 px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                    errors.sourceVideoUrl ? 'border-red-500' : 'border-gray-300'
                  }`}
                />
                <button
                  type="button"
                  onClick={() => handlePasteUrl('source')}
                  className="px-3 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors text-sm"
                  title="從剪貼簿貼上"
                >
                  貼上
                </button>
              </div>
              {errors.sourceVideoUrl && (
                <p className="mt-1 text-sm text-red-600">{errors.sourceVideoUrl}</p>
              )}
            </div>

            <TimeSelector
              value={{ start: formData.sourceStartTime, end: formData.sourceEndTime }}
              onChange={handleSourceTimeChange}
              maxDuration={currentVideoDuration}
              className="border border-gray-200 rounded-lg p-4 bg-white/80 backdrop-blur-sm"
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
                  ref={targetUrlRef}
                  type="url"
                  value={formData.targetVideoUrl}
                  onChange={(e) => setFormData(prev => ({ ...prev, targetVideoUrl: e.target.value }))}
                  onKeyDown={(e) => handleKeyDown(e, 'target')}
                  placeholder="https://www.youtube.com/watch?v=... (Ctrl+V 貼上)"
                  className={`flex-1 px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                    errors.targetVideoUrl ? 'border-red-500' : 'border-gray-300'
                  }`}
                />
                <button
                  type="button"
                  onClick={() => handlePasteUrl('target')}
                  className="px-3 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors text-sm"
                  title="從剪貼簿貼上"
                >
                  貼上
                </button>
              </div>
              {errors.targetVideoUrl && (
                <p className="mt-1 text-sm text-red-600">{errors.targetVideoUrl}</p>
              )}
            </div>

            <TimeSelector
              value={{ start: formData.targetStartTime, end: formData.targetEndTime }}
              onChange={handleTargetTimeChange}
              maxDuration={currentVideoDuration}
              className="border border-gray-200 rounded-lg p-4 bg-white/80 backdrop-blur-sm"
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