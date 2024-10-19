import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { z } from "zod";

function useValidatedQueryParams<T extends z.ZodRawShape>(
  schema: z.ZodObject<T>
) {
  const [rawParams] = useSearchParams();
  const [validation, setValidation] = useState(
    schema.safeParse(Object.fromEntries(rawParams.entries()))
  );

  useEffect(() => {
    setValidation(schema.safeParse(Object.fromEntries(rawParams.entries())));
  }, [rawParams, schema]);

  return validation;
}

export default useValidatedQueryParams;
