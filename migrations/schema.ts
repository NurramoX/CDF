import { text, sqliteTable } from 'drizzle-orm/sqlite-core';

export const fast_forwards = sqliteTable('fast_forwards', {
  key: text('fast_forward').primaryKey(),
  path: text('path').notNull(),
});
