"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { listen } from "@tauri-apps/api/event";

/**
 * Bridges tray-menu clicks to client-side routing: the Rust tray emits a
 * `navigate` event with a route string, which we push onto the Next router.
 */
export function TrayNavigation() {
  const router = useRouter();

  useEffect(() => {
    const unlisten = listen<string>("navigate", (event) => {
      router.push(event.payload);
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [router]);

  return null;
}
