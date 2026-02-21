import { useRef, useEffect, useState } from "react";

interface LogEntry {
  type: "stdout" | "stderr" | "system";
  text: string;
  timestamp: Date;
}

interface LauncherProps {
  onStart: () => Promise<void>;
  logs: LogEntry[];
  onClearLogs: () => void;
  wasRunning: boolean;
}

export default function Launcher({ onStart, logs, onClearLogs, wasRunning }: LauncherProps) {
  const [starting, setStarting] = useState(false);
  const logPanelOpen = true;
  const logContainerRef = useRef<HTMLDivElement>(null);

  // 自动滚动到日志底部
  useEffect(() => {
    if (logContainerRef.current) {
      logContainerRef.current.scrollTop = logContainerRef.current.scrollHeight;
    }
  }, [logs]);

  const handleStart = async () => {
    setStarting(true);
    try {
      await onStart();
    } catch (error) {
      console.error("启动失败:", error);
    } finally {
      setStarting(false);
    }
  };

  return (
    <div className="flex h-full flex-col bg-neutral-900">
      {/* 主内容区域 */}
      <div className="flex-1 flex items-center justify-center p-8">
        <div className="w-full max-w-md space-y-6 rounded-lg border border-neutral-800 bg-neutral-950 p-8">
          <h1 className="text-3xl font-bold text-white text-center">
            MCP Inspector Desktop
          </h1>

          <p className="text-sm text-neutral-400 text-center">
            {wasRunning ? "Inspector 已停止" : "准备就绪"}
          </p>

          {/* 状态指示 */}
          <div className="flex items-center justify-center gap-2 mb-6">
            <span className={`h-3 w-3 rounded-full ${wasRunning ? "bg-red-500" : "bg-green-500"}`} />
            <span className="text-sm text-neutral-300">
              {wasRunning ? "已停止" : "准备就绪"}
            </span>
          </div>

          {/* 启动按钮 */}
          <button
            onClick={handleStart}
            disabled={starting}
            className="w-full rounded bg-blue-600 py-3 text-lg font-semibold text-white hover:bg-blue-700 disabled:bg-neutral-700 disabled:text-neutral-500"
          >
            {starting ? "启动中..." : "启动 Inspector"}
          </button>
        </div>
      </div>

      {/* 日志面板 */}
      {logPanelOpen && (
        <div className="flex h-64 flex-col border-t border-neutral-800 bg-neutral-950">
          {/* 日志面板头部 */}
          <div className="flex items-center justify-between border-b border-neutral-800 bg-neutral-900 px-4 py-2">
            <span className="text-sm font-medium text-neutral-300">
              启动日志
            </span>
            <button
              onClick={onClearLogs}
              className="text-xs text-neutral-400 hover:text-neutral-200"
            >
              清空日志
            </button>
          </div>

          {/* 日志内容 */}
          <div
            ref={logContainerRef}
            className="flex-1 overflow-auto p-4 font-mono text-xs"
          >
            {logs.length === 0 ? (
              <div className="text-neutral-500">等待日志输出...</div>
            ) : (
              logs.map((log, index) => (
                <div
                  key={index}
                  className={`mb-1 ${
                    log.type === "stderr"
                      ? "text-red-400"
                      : log.type === "system"
                      ? "text-blue-400"
                      : "text-neutral-300"
                  }`}
                >
                  <span className="text-neutral-600">
                    [{log.timestamp.toLocaleTimeString()}]
                  </span>{" "}
                  {log.text}
                </div>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
}
