import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

interface LauncherProps {
  onStart: (config: {
    command: string;
    workingDir: string;
    envVars: Record<string, string>;
  }) => Promise<void>;
}

export default function Launcher({ onStart }: LauncherProps) {
  const [command, setCommand] = useState("node ./dist/index.js");
  const [workingDir, setWorkingDir] = useState("");
  const [envVars, setEnvVars] = useState<Record<string, string>>({});
  const [newEnvKey, setNewEnvKey] = useState("");
  const [newEnvValue, setNewEnvValue] = useState("");
  const [starting, setStarting] = useState(false);

  const handleSelectWorkingDir = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected && typeof selected === "string") {
      setWorkingDir(selected);
    }
  };

  const handleAddEnvVar = () => {
    if (newEnvKey && newEnvValue) {
      setEnvVars({ ...envVars, [newEnvKey]: newEnvValue });
      setNewEnvKey("");
      setNewEnvValue("");
    }
  };

  const handleRemoveEnvVar = (key: string) => {
    const newVars = { ...envVars };
    delete newVars[key];
    setEnvVars(newVars);
  };

  const handleStart = async () => {
    if (!workingDir) {
      alert("请选择工作目录");
      return;
    }
    setStarting(true);
    try {
      await onStart({ command, workingDir, envVars });
    } catch (error) {
      alert(`启动失败: ${error}`);
    } finally {
      setStarting(false);
    }
  };

  return (
    <div className="flex h-full items-center justify-center bg-neutral-900 p-8">
      <div className="w-full max-w-2xl space-y-6 rounded-lg border border-neutral-800 bg-neutral-950 p-8">
        <h1 className="text-3xl font-bold text-white">
          MCP Inspector Desktop
        </h1>

        {/* 服务器命令 */}
        <div className="space-y-2">
          <label className="block text-sm font-medium text-neutral-300">
            服务器命令
          </label>
          <input
            type="text"
            value={command}
            onChange={(e) => setCommand(e.target.value)}
            className="w-full rounded border border-neutral-700 bg-neutral-900 px-3 py-2 text-white focus:border-blue-500 focus:outline-none"
            placeholder="例如: node ./dist/index.js"
          />
        </div>

        {/* 工作目录 */}
        <div className="space-y-2">
          <label className="block text-sm font-medium text-neutral-300">
            工作目录
          </label>
          <div className="flex gap-2">
            <input
              type="text"
              value={workingDir}
              onChange={(e) => setWorkingDir(e.target.value)}
              className="flex-1 rounded border border-neutral-700 bg-neutral-900 px-3 py-2 text-white focus:border-blue-500 focus:outline-none"
              placeholder="选择服务器项目目录"
              readOnly
            />
            <button
              onClick={handleSelectWorkingDir}
              className="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
            >
              浏览
            </button>
          </div>
        </div>

        {/* 环境变量 */}
        <div className="space-y-2">
          <label className="block text-sm font-medium text-neutral-300">
            环境变量
          </label>
          <div className="space-y-2">
            {Object.entries(envVars).map(([key, value]) => (
              <div
                key={key}
                className="flex items-center gap-2 rounded border border-neutral-700 bg-neutral-900 p-2"
              >
                <span className="flex-1 font-mono text-sm text-neutral-300">
                  {key}={value}
                </span>
                <button
                  onClick={() => handleRemoveEnvVar(key)}
                  className="text-red-400 hover:text-red-300"
                >
                  删除
                </button>
              </div>
            ))}
            <div className="flex gap-2">
              <input
                type="text"
                value={newEnvKey}
                onChange={(e) => setNewEnvKey(e.target.value)}
                className="w-1/3 rounded border border-neutral-700 bg-neutral-900 px-3 py-2 text-white focus:border-blue-500 focus:outline-none"
                placeholder="变量名"
              />
              <input
                type="text"
                value={newEnvValue}
                onChange={(e) => setNewEnvValue(e.target.value)}
                className="flex-1 rounded border border-neutral-700 bg-neutral-900 px-3 py-2 text-white focus:border-blue-500 focus:outline-none"
                placeholder="变量值"
              />
              <button
                onClick={handleAddEnvVar}
                className="rounded bg-neutral-700 px-4 py-2 text-white hover:bg-neutral-600"
              >
                添加
              </button>
            </div>
          </div>
        </div>

        {/* 启动按钮 */}
        <button
          onClick={handleStart}
          disabled={starting || !workingDir}
          className="w-full rounded bg-blue-600 py-3 text-lg font-semibold text-white hover:bg-blue-700 disabled:bg-neutral-700 disabled:text-neutral-500"
        >
          {starting ? "启动中..." : "启动 Inspector"}
        </button>
      </div>
    </div>
  );
}
