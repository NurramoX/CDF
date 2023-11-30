import { migrate } from 'drizzle-orm/better-sqlite3/migrator';
import { drizzle } from 'drizzle-orm/better-sqlite3';
import Database from 'better-sqlite3';
import os from 'os';
import path from 'path';
import { promises as fs } from 'fs';

const homeDirectory = os.homedir();
const localSharePath = path.join(homeDirectory, '.local', 'share', 'cdf');
const sqlitePath = path.join(localSharePath, 'sqlite.db');

await fs.mkdir(localSharePath).catch((err) => {
  if (err.code === 'EEXIST') {
    console.log('Folder already exists hehe');
  } else {
    throw err;
  }
});

const sqlite = new Database(sqlitePath);
const db = drizzle(sqlite);
migrate(db, { migrationsFolder: './drizzle' });
