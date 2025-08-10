import { z } from 'zod';

export const characterSchema = z.object({
  name: z.string(),
  creator: z.string(),
});

export type CharacterFormData = z.infer<typeof characterSchema>;
