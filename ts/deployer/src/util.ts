import { mkdirSync, readFileSync, rmSync, writeFileSync } from "fs";
import { join } from "path";
import { NETWORK } from "./config";

export function ExportReport(subject: string, output: any) {
  const base = join(process.cwd(), "out", subject, NETWORK);
  mkdirSync(base, { recursive: true });

  const fileLatest = "run-latest.json";
  rmSync(join(base, fileLatest), { force: true });

  const fileTime = `run-${Math.floor(new Date().getTime() / 1000)}.json`;

  const fileData = JSON.stringify(output, null, 2);
  writeFileSync(join(base, fileLatest), fileData);
  writeFileSync(join(base, fileTime), fileData);
}

export function LoadReport<T>(subject: string): T {
  const base = join(process.cwd(), "out", subject, NETWORK);

  return JSON.parse(readFileSync(join(base, "run-latest.json"), "utf-8"));
}
