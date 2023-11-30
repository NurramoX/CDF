import type { Config } from 'drizzle-kit';

export default {
  schema: './migrations/schema.ts',
  out: 'drizzle',
} satisfies Config;
