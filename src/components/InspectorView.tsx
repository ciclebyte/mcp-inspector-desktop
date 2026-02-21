import { useState, useEffect, useRef } from "react";

interface InspectorViewProps {
  url: string;
  onStop: () => Promise<void>;
  logs: Array<{
    type: "stdout" | "stderr" | "system";
    text: string;
    timestamp: Date;
  }>;
  onClearLogs: () => void;
}

export default function InspectorView({ url, onStop, logs, onClearLogs }: InspectorViewProps) {
  const [stopping, setStopping] = useState(false);
  const [logPanelOpen, setLogPanelOpen] = useState(false);
  const logContainerRef = useRef<HTMLDivElement>(null);

  // 调试：打印 URL 变化
  useEffect(() => {
    console.log("InspectorView URL 更新:", url);
  }, [url]);

  // 自动滚动到日志底部
  useEffect(() => {
    if (logContainerRef.current) {
      logContainerRef.current.scrollTop = logContainerRef.current.scrollHeight;
    }
  }, [logs]);

  const handleStop = async () => {
    setStopping(true);
    try {
      await onStop();
    } finally {
      setStopping(false);
    }
  };

  return (
    <div className="flex h-full flex-col">
      {/* 顶部工具栏 */}
      <div className="flex items-center justify-between border-b border-neutral-800 bg-neutral-900 px-4 py-2">
        <h1 className="text-lg font-semibold text-white">
          MCP Inspector Desktop
        </h1>
        <div className="flex items-center gap-4">
          <span className="flex items-center gap-2 text-sm text-neutral-400">
            <span className="h-2 w-2 rounded-full bg-green-500" />
            运行中
          </span>
          <button
            onClick={() => setLogPanelOpen(!logPanelOpen)}
            className="rounded bg-neutral-700 px-3 py-1.5 text-sm font-medium text-white hover:bg-neutral-600"
          >
            {logPanelOpen ? "隐藏日志" : "显示日志"}
          </button>
          <button
            onClick={handleStop}
            disabled={stopping}
            className="rounded bg-red-600 px-4 py-1.5 text-sm font-medium text-white hover:bg-red-700 disabled:bg-neutral-700"
          >
            {stopping ? "停止中..." : "停止"}
          </button>
        </div>
      </div>

      {/* 主内容区域 */}
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Inspector iframe */}
        <div className={logPanelOpen ? "flex-1" : "flex-1"}>
          <iframe
            key={url}  // 强制在 URL 变化时重新加载
            src={url}
            className="h-full w-full border-0"
            title="MCP Inspector"
          />
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
    </div>
  );
}
