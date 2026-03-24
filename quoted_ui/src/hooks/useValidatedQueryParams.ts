import { useMemo } from "react";
import { useSearchParams } from "react-router-dom";
import { z } from "zod";

export function useValidatedQueryParams<T extends z.ZodRawShape>(
  schema: z.ZodObject<T>,
): [
  z.infer<typeof schema> | null,
  (updates: Partial<z.infer<typeof schema>>) => void,
] {
  const [searchParams, setSearchParams] = useSearchParams();

  const validation = useMemo(() => {
    return schema.safeParse(Object.fromEntries(searchParams.entries()));
  }, [searchParams, schema]);

  const values = validation.success ? validation.data : null;

  function setValues(updates: Partial<z.infer<typeof schema>>) {
    const newParams = new URLSearchParams(searchParams);

    Object.entries(updates).forEach(([key, value]) => {
      if (value === undefined || value === null || value === "") {
        newParams.delete(key);
      } else {
        newParams.set(key, String(value));
      }
    });

    setSearchParams(newParams);
  }

  return [values, setValues];
}

export default useValidatedQueryParams;
