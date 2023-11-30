#!/usr/bin/env vite-node --script

import { BetterSQLite3Database, drizzle } from 'drizzle-orm/better-sqlite3';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import Database from 'better-sqlite3';
import os from 'os';
import path from 'path';
import * as schema from '../migrations/schema';
import { fast_forwards } from '../migrations/schema';
import { eq, like } from 'drizzle-orm';
import * as process from 'process';
import * as console from 'console';

const isVerbose =
  process.argv.includes('-v') || process.argv.includes('--verbose');

function logger(message: string) {
  if (isVerbose) {
    console.log(message);
  }
}

yargs(hideBin(process.argv))
  .option('verbose', {
    alias: 'v',
    type: 'boolean',
    description: 'Run with verbose logging',
  })
  .command(
    '$0 <abbr>',
    'Default command to get path from abbr',
    (yargs) => {
      return yargs.positional('abbr', {
        describe: 'The path to process',
        type: 'string',
      });
    },
    (argv) => {
      const db = connectToDatabase();
      logger(`Processing path: ${argv.abbr}`);
      handleDefault(db, argv.abbr!);
    },
  )
  .command(
    'register <abbr> <path>',
    'Register a new path',
    (yargs) => {
      return yargs
        .positional('abbr', {
          describe: 'Abbreviation of path to be registered',
          type: 'string',
        })
        .positional('path', {
          describe: 'Actual path to be registered',
          type: 'string',
        });
    },
    (argv) => {
      const db = connectToDatabase();
      logger(`Registering path: ${argv.path}`);
      // Your register functionality here
      handleRegister(db, argv.abbr!, argv.path!);
    },
  )
  .command(
    'query <subabbr>',
    'Query an existing path',
    (yargs) => {
      return yargs.positional('subabbr', {
        describe: 'Query existing abbreviations based on sub abbreviation',
        type: 'string',
      });
    },
    (argv) => {
      const db = connectToDatabase();
      logger(`Querying path: ${argv.subabbr}`);
      handleQuery(db, argv.subabbr!);
    },
  )
  .strictCommands()
  .help().argv;

function handleRegister(
  db: BetterSQLite3Database<typeof schema>,
  key: string,
  path: string,
) {
  db.insert(schema.fast_forwards)
    .values({ key, path })
    .then(() => {
      logger('success');
      process.exit(0);
    })
    .catch((x) => {
      console.error(x);
      process.exit(1);
    });
}

function handleDefault(db: BetterSQLite3Database<typeof schema>, key: string) {
  db.query.fast_forwards
    .findFirst({
      where: eq(fast_forwards.key, key),
    })
    .then(
      (x) => {
        if (x === undefined) {
          console.error(`There is no path that is registered under ${key}`);
          process.exit(1);
        }
        console.log(x.path);
        process.exit(0);
      },
      (err) => {
        console.error(err);
        process.exit(1);
      },
    );
}

function handleQuery(
  db: BetterSQLite3Database<typeof schema>,
  subabbr: string,
) {
  const query = db
    .select({
      abbr: fast_forwards.key,
    })
    .from(fast_forwards)
    .where(like(fast_forwards.key, `%${subabbr}%`));
  query
    .then((result) => {
      console.log(result.map((x) => x.abbr).join('\n'));
    })
    .catch((err) => {
      console.error(err);
      process.exit(1);
    });
}

function connectToDatabase() {
  logger(`Connecting to database!`);
  const homeDirectory = os.homedir();
  const sqlitePath = path.join(
    homeDirectory,
    '.local',
    'share',
    'cdf',
    'sqlite.db',
  );
  const sqlite = new Database(sqlitePath);
  logger(`Connected!`);
  return drizzle(sqlite, { schema });
}
