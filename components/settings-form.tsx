"use client";

import { useEffect, useState, type FormEvent } from "react";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { loadKeys, saveKeys, type ApiKeyName } from "@/lib/secrets";

const keysSchema = z.object({
  anthropic: z.string().trim().min(1, "Required"),
  deepgram: z.string().trim().min(1, "Required"),
  voyage: z.string().trim().min(1, "Required"),
});

type Keys = z.infer<typeof keysSchema>;

const FIELDS = [
  { id: "anthropic", label: "Anthropic API key" },
  { id: "deepgram", label: "Deepgram API key" },
  { id: "voyage", label: "Voyage AI API key" },
] as const satisfies ReadonlyArray<{ id: ApiKeyName; label: string }>;

const EMPTY: Keys = { anthropic: "", deepgram: "", voyage: "" };

export function SettingsForm() {
  const [values, setValues] = useState<Keys>(EMPTY);
  const [errors, setErrors] = useState<Partial<Record<keyof Keys, string>>>({});
  const [status, setStatus] = useState<"idle" | "loading" | "saving" | "saved">(
    "loading",
  );
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    loadKeys()
      .then((stored) => {
        if (active) {
          setValues({ ...EMPTY, ...stored });
          setStatus("idle");
        }
      })
      .catch((err: unknown) => {
        if (active) {
          setLoadError(err instanceof Error ? err.message : String(err));
          setStatus("idle");
        }
      });
    return () => {
      active = false;
    };
  }, []);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setStatus("idle");

    const result = keysSchema.safeParse(values);
    if (!result.success) {
      const fieldErrors: Partial<Record<keyof Keys, string>> = {};
      for (const issue of result.error.issues) {
        fieldErrors[issue.path[0] as keyof Keys] = issue.message;
      }
      setErrors(fieldErrors);
      return;
    }

    setErrors({});
    setStatus("saving");
    try {
      await saveKeys(result.data);
      setStatus("saved");
      setLoadError(null);
    } catch (err: unknown) {
      setStatus("idle");
      setLoadError(err instanceof Error ? err.message : String(err));
    }
  }

  return (
    <form onSubmit={handleSubmit} className="w-full max-w-md space-y-6">
      <div className="space-y-1">
        <h1 className="text-2xl font-semibold">Settings</h1>
        <p className="text-sm text-zinc-500">
          API keys are encrypted and stored only on this machine.
        </p>
      </div>

      {loadError ? (
        <p className="text-sm text-red-600">{loadError}</p>
      ) : null}

      <div className="space-y-4">
        {FIELDS.map((field) => (
          <div key={field.id} className="space-y-1.5">
            <Label htmlFor={field.id}>{field.label}</Label>
            <Input
              id={field.id}
              type="password"
              autoComplete="off"
              spellCheck={false}
              value={values[field.id]}
              onChange={(event) =>
                setValues((prev) => ({ ...prev, [field.id]: event.target.value }))
              }
            />
            {errors[field.id] ? (
              <p className="text-sm text-red-600">{errors[field.id]}</p>
            ) : null}
          </div>
        ))}
      </div>

      <div className="flex items-center gap-3">
        <Button type="submit" disabled={status === "saving"}>
          {status === "saving" ? "Saving…" : "Save"}
        </Button>
        {status === "saved" ? (
          <span className="text-sm text-green-600">Saved</span>
        ) : null}
      </div>
    </form>
  );
}
