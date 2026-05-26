"use client";

import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

/**
 * Bridges the `record-toggle` hotkey event to the capture commands. Tracks
 * recording state in a ref and reconciles it from the authoritative
 * `capture-status` event Rust emits. (User-facing error surfacing is S2-05.)
 */
export function CaptureController() {
  const recording = useRef(false);

  useEffect(() => {
    const unToggle = listen("record-toggle", () => {
      void (async () => {
        if (recording.current) {
          recording.current = false;
          try {
            await invoke("stop_capture");
          } catch {
            recording.current = false;
          }
        } else {
          recording.current = true;
          try {
            await invoke("start_capture");
          } catch {
            recording.current = false;
          }
        }
      })();
    });

    const unStatus = listen<string>("capture-status", (event) => {
      recording.current = event.payload === "recording";
    });

    return () => {
      void unToggle.then((fn) => fn());
      void unStatus.then((fn) => fn());
    };
  }, []);

  return null;
}
