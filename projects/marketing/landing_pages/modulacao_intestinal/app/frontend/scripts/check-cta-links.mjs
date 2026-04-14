import { readdirSync, readFileSync, statSync } from "node:fs";
import { extname, join } from "node:path";

const COMPONENTS_DIR = "src/components";
const ALLOWED_EXTENSIONS = new Set([".ts", ".tsx", ".js", ".jsx"]);

const WHATSAPP_PATTERNS = [
  /href\s*=\s*["'`]https?:\/\/(?:wa\.me|api\.whatsapp\.com|(?:www\.)?whatsapp\.com\/send)/gi,
  /["'`]https?:\/\/(?:wa\.me|api\.whatsapp\.com|(?:www\.)?whatsapp\.com\/send)/gi,
];

const getFiles = (dir) => {
  const entries = readdirSync(dir);
  const files = [];

  for (const entry of entries) {
    const path = join(dir, entry);
    const stats = statSync(path);

    if (stats.isDirectory()) {
      files.push(...getFiles(path));
      continue;
    }

    if (ALLOWED_EXTENSIONS.has(extname(path))) {
      files.push(path);
    }
  }

  return files;
};

const linesWithMatches = [];

for (const file of getFiles(COMPONENTS_DIR)) {
  const content = readFileSync(file, "utf8");
  const lines = content.split(/\r?\n/);

  lines.forEach((line, index) => {
    const hasWhatsappLink = WHATSAPP_PATTERNS.some((pattern) => pattern.test(line));

    WHATSAPP_PATTERNS.forEach((pattern) => {
      pattern.lastIndex = 0;
    });

    if (hasWhatsappLink) {
      linesWithMatches.push(`${file}:${index + 1}: ${line.trim()}`);
    }
  });
}

if (linesWithMatches.length > 0) {
  console.error("❌ Foram encontrados links diretos de WhatsApp em src/components.");
  console.error("Use sempre CTA_URL de src/lib/cta.ts para CTAs externos.");
  console.error("");
  linesWithMatches.forEach((line) => console.error(`- ${line}`));
  process.exit(1);
}

console.log("✅ Verificação de CTAs aprovada: nenhum link direto de WhatsApp encontrado em src/components.");
