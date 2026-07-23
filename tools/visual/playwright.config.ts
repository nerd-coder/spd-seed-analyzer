import { defineConfig } from '@playwright/test'

const PORT = 4173
const BASE_URL = `http://127.0.0.1:${PORT}`

export default defineConfig({
  testDir: './tests',
  testMatch: '**/*.spec.ts',
  outputDir: './test-results',
  snapshotPathTemplate: '{testDir}/../snapshots/{arg}{ext}',
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: 1,
  reporter: 'line',
  timeout: 120_000,
  expect: {
    timeout: 15_000,
    toMatchSnapshot: {
      maxDiffPixels: 0,
      threshold: 0,
    },
    toHaveScreenshot: {
      animations: 'disabled',
      maxDiffPixels: 0,
      scale: 'css',
      threshold: 0,
    },
  },
  use: {
    baseURL: BASE_URL,
    browserName: 'chromium',
    colorScheme: 'light',
    deviceScaleFactor: 1,
    locale: 'en-US',
    reducedMotion: 'reduce',
    screenshot: 'only-on-failure',
    trace: 'retain-on-failure',
    timezoneId: 'UTC',
    viewport: { width: 1440, height: 1200 },
  },
  webServer: {
    command: `bun run preview -- --host 127.0.0.1 --port ${PORT}`,
    cwd: '../..',
    url: BASE_URL,
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
  },
})
