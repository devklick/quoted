import { z } from "zod";

export const quotesQueryParamsSchema = z.object({
  showName: z.string().optional(),
  seasonNo: z.string().pipe(z.coerce.number()).optional(),
  episodeNo: z.string().pipe(z.coerce.number()).optional(),
});

export type QuotesQueryParams = z.infer<typeof quotesQueryParamsSchema>;
