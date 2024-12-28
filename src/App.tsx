import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import "./App.css";


interface ProcessInfo {
  id: string,
  name: string
}

const App: React.FC = () => {
  const [osName, setOsName] = useState<String>();
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);

  useEffect(() => {
    async function fetchData() {
      const os = await invoke<string>("os_name");
      const processList = await invoke<ProcessInfo[]>("list_process");
      setOsName(os);
      setProcesses(processList);
    }

    fetchData();
  }, []);

  async function deleteProcess(id: String) {
    const success = await invoke<boolean>("kill_by_id", { id });

    if (success) {
      setProcesses((prevProcesses) => prevProcesses.filter((p) => p.id !== id));
    }
  } 

  return (
    <main className="container">
      <h2>Operating System: {osName}</h2>
      <div className="process-list">
        {
          processes.map((process) => (
            <div key={process.id} className="process-item">
              <span>{process.name} (ID: {process.id})</span>
              <button onClick={() => deleteProcess(process.id)}>
                Kill Process
              </button>
            </div>

          ))
        }
      </div>
    </main>
  );
};

export default App;