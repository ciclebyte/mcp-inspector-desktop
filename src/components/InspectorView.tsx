import { useState } from "react";

interface InspectorViewProps {
  url: string;
  onStop: () => Promise<void>;
}

export default function InspectorView({ url, onStop }: InspectorViewProps) {
  const [stopping, setStopping] = useState(false);

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
            onClick={handleStop}
            disabled={stopping}
            className="rounded bg-red-600 px-4 py-1.5 text-sm font-medium text-white hover:bg-red-700 disabled:bg-neutral-700"
          >
            {stopping ? "停止中..." : "停止"}
          </button>
        </div>
      </div>

      {/* Inspector iframe */}
      <div className="flex-1">
        <iframe
          src={url}
          className="h-full w-full border-0"
          title="MCP Inspector"
        />
      </div>
    </div>
  );
}
