import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Launcher from "./components/Launcher";
import InspectorView from "./components/InspectorView";

interface InspectorStatus {
  running: boolean;
  url?: string;
}

function App() {
  const [inspectorStatus, setInspectorStatus] = useState<InspectorStatus>({
    running: false,
  });

  useEffect(() => {
    // 监听 Inspector 就绪事件
    const unlisten = listen<number>("inspector-ready", (event) => {
      setInspectorStatus({
        running: true,
        url: `http://localhost:${event.payload}`,
      });
    });

    // 检查当前状态
    invoke<string>("get_inspector_status")
      .then((url) => {
        if (url) {
          setInspectorStatus({ running: true, url });
        }
      })
      .catch(() => {
        // 忽略错误
      });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const handleStart = async (config: {
    command: string;
    workingDir: string;
    envVars: Record<string, string>;
  }) => {
    try {
      const port = await invoke<number>("start_inspector", {
        command: config.command,
        workingDir: config.workingDir,
        envVars: config.envVars,
      });
      setInspectorStatus({
        running: true,
        url: `http://localhost:${port}`,
      });
    } catch (error) {
      console.error("Failed to start inspector:", error);
      throw error;
    }
  };

  const handleStop = async () => {
    try {
      await invoke("stop_inspector");
      setInspectorStatus({ running: false });
    } catch (error) {
      console.error("Failed to stop inspector:", error);
      throw error;
    }
  };

  return (
    <div className="h-screen w-screen bg-background text-foreground">
      {!inspectorStatus.running ? (
        <Launcher onStart={handleStart} />
      ) : (
        <InspectorView url={inspectorStatus.url} onStop={handleStop} />
      )}
    </div>
  );
}

export default App;
