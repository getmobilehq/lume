"use client";

import { useEffect, useState, type ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

/**
 * Gates its children on macOS Screen Recording permission. Tauri can't observe
 * the grant live, so when it's missing we show setup instructions and a Restart
 * button rather than the app content.
 */
export function PermissionGate({ children }: { children: ReactNode }) {
  const [granted, setGranted] = useState<boolean | null>(null);

  useEffect(() => {
    let active = true;
    invoke<boolean>("screen_recording_permission")
      .then((ok) => active && setGranted(ok))
      .catch(() => active && setGranted(true)); // non-Tauri/dev fallback: don't block
    return () => {
      active = false;
    };
  }, []);

  if (granted === null) return null;
  if (granted) return <>{children}</>;

  return (
    <main className="flex flex-1 flex-col items-center justify-center gap-5 p-8 text-center">
      <div className="space-y-2">
        <h1 className="text-2xl font-semibold">Allow Screen Recording</h1>
        <p className="max-w-md text-sm text-zinc-500">
          Lume records your active browser window. macOS needs Screen Recording
          permission before the first capture.
        </p>
      </div>

      <ol className="max-w-md space-y-1 text-left text-sm text-zinc-600">
        <li>1. Click <span className="font-medium">Open Settings</span> below.</li>
        <li>2. Enable <span className="font-medium">Lume</span> under Screen &amp; System Audio Recording.</li>
        <li>3. Click <span className="font-medium">Restart Lume</span> to apply.</li>
      </ol>

      <div className="flex items-center gap-3">
        <Button
          variant="outline"
          onClick={() => void invoke("request_screen_recording_permission")}
        >
          Open Settings
        </Button>
        <Button onClick={() => void invoke("restart_app")}>Restart Lume</Button>
      </div>
    </main>
  );
}
