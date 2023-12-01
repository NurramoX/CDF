import esbuild from 'esbuild';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const out = 'dist';

await esbuild
  .build({
    entryPoints: ['src/main.ts', 'migrations/schema.ts'],
    outdir: out,
    bundle: false,
    platform: 'node',
    format: 'esm',
  })
  .catch(() => process.exit(1));

// Use fileURLToPath and import.meta.url to get __dirname equivalent
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const directoryPath = path.join(__dirname, out);

const addJsExtensionToImports = (filePath) => {
  let data = fs.readFileSync(filePath, 'utf-8');
  const regex = /from ['"](\..*?)['"];/g;

  data = data.replace(regex, (match, p1) => {
    if (!p1.endsWith('.js')) {
      return match.replace(p1, p1 + '.js');
    }
    return match;
  });

  fs.writeFileSync(filePath, data, 'utf-8');
};

const processDirectory = (dirPath) => {
  fs.readdirSync(dirPath).forEach((file) => {
    const absolutePath = path.join(dirPath, file);
    if (fs.statSync(absolutePath).isDirectory()) {
      processDirectory(absolutePath);
    } else if (absolutePath.endsWith('.js')) {
      addJsExtensionToImports(absolutePath);
    }
  });
};

processDirectory(directoryPath);
