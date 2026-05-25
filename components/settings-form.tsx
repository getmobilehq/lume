"use client";

import { useState, type FormEvent } from "react";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

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
] as const satisfies ReadonlyArray<{ id: keyof Keys; label: string }>;

const EMPTY: Keys = { anthropic: "", deepgram: "", voyage: "" };

export function SettingsForm() {
  const [values, setValues] = useState<Keys>(EMPTY);
  const [errors, setErrors] = useState<Partial<Record<keyof Keys, string>>>({});
  const [saved, setSaved] = useState(false);

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSaved(false);

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
    // Persistence (Keychain via stronghold) lands in wk1-foundation-stronghold;
    // for now the validated keys live only in component state.
    setSaved(true);
  }

  return (
    <form onSubmit={handleSubmit} className="w-full max-w-md space-y-6">
      <div className="space-y-1">
        <h1 className="text-2xl font-semibold">Settings</h1>
        <p className="text-sm text-zinc-500">
          API keys are stored only on this machine.
        </p>
      </div>

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
        <Button type="submit">Save</Button>
        {saved ? <span className="text-sm text-green-600">Saved</span> : null}
      </div>
    </form>
  );
}
