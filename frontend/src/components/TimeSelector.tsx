import React, { useState, useEffect } from 'react';
import type { TimeRange } from '../types';
import { formatTime } from '../utils/youtube';

interface TimeSelectorProps {
  value: TimeRange;
  onChange: (range: TimeRange) => void;
  maxDuration: number;
  className?: string;
}

export const TimeSelector: React.FC<TimeSelectorProps> = ({
  value,
  onChange,
  maxDuration,
  className = ''
}) => {
  const [startTime, setStartTime] = useState(value.start);
  const [endTime, setEndTime] = useState(value.end);

  useEffect(() => {
    setStartTime(value.start);
    setEndTime(value.end);
  }, [value]);

  const handleStartTimeChange = (newStartTime: number) => {
    const clampedStartTime = Math.max(0, Math.min(newStartTime, endTime - 1));
    setStartTime(clampedStartTime);
    onChange({ start: clampedStartTime, end: endTime });
  };

  const handleEndTimeChange = (newEndTime: number) => {
    const clampedEndTime = Math.max(startTime + 1, Math.min(newEndTime, maxDuration));
    setEndTime(clampedEndTime);
    onChange({ start: startTime, end: clampedEndTime });
  };

  const handleCurrentTimeClick = () => {
    // 取得當前播放時間
    const video = document.querySelector('video') as HTMLVideoElement;
    if (video) {
      const currentTime = Math.floor(video.currentTime);
      handleStartTimeChange(currentTime);
    }
  };

  const handleSetEndToCurrent = () => {
    const video = document.querySelector('video') as HTMLVideoElement;
    if (video) {
      const currentTime = Math.floor(video.currentTime);
      handleEndTimeChange(currentTime);
    }
  };

  return (
    <div className={`space-y-4 ${className}`}>
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-gray-700">時間範圍</h3>
        <span className="text-xs text-gray-500">
          總時長: {formatTime(maxDuration)}
        </span>
      </div>

      <div className="space-y-3">
        {/* 開始時間 */}
        <div className="flex items-center space-x-2">
          <label className="text-sm font-medium text-gray-600 w-16">
            開始:
          </label>
          <input
            type="range"
            min="0"
            max={maxDuration}
            value={startTime}
            onChange={(e) => handleStartTimeChange(Number(e.target.value))}
            className="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
          />
          <span className="text-sm text-gray-700 w-12">
            {formatTime(startTime)}
          </span>
          <button
            type="button"
            onClick={handleCurrentTimeClick}
            className="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
          >
            當前
          </button>
        </div>

        {/* 結束時間 */}
        <div className="flex items-center space-x-2">
          <label className="text-sm font-medium text-gray-600 w-16">
            結束:
          </label>
          <input
            type="range"
            min="0"
            max={maxDuration}
            value={endTime}
            onChange={(e) => handleEndTimeChange(Number(e.target.value))}
            className="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
          />
          <span className="text-sm text-gray-700 w-12">
            {formatTime(endTime)}
          </span>
          <button
            type="button"
            onClick={handleSetEndToCurrent}
            className="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
          >
            當前
          </button>
        </div>
      </div>

      {/* 時間範圍顯示 */}
      <div className="bg-gray-50 p-3 rounded-lg">
        <div className="text-sm text-gray-600">
          選擇範圍: {formatTime(startTime)} - {formatTime(endTime)}
        </div>
        <div className="text-xs text-gray-500 mt-1">
          持續時間: {formatTime(endTime - startTime)}
        </div>
      </div>

      {/* 快速選擇按鈕 */}
      <div className="flex space-x-2">
        <button
          type="button"
          onClick={() => onChange({ start: 0, end: Math.min(30, maxDuration) })}
          className="px-3 py-1 text-xs bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors"
        >
          前30秒
        </button>
        <button
          type="button"
          onClick={() => onChange({ start: Math.max(0, maxDuration - 30), end: maxDuration })}
          className="px-3 py-1 text-xs bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors"
        >
          後30秒
        </button>
        <button
          type="button"
          onClick={() => onChange({ start: 0, end: maxDuration })}
          className="px-3 py-1 text-xs bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors"
        >
          全片
        </button>
      </div>
    </div>
  );
}; 