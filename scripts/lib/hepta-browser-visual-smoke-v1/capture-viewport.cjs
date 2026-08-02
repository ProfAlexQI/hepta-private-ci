const { chromium } = require("playwright");

const [chromeBin, baseUrl, widthRaw, heightRaw, screenshotPath] = process.argv.slice(2);
const width = Number(widthRaw);
const height = Number(heightRaw);

(async () => {
  const browser = await chromium.launch({
    headless: true,
    executablePath: chromeBin,
    args: [
      "--disable-background-networking",
      "--disable-component-update",
      "--disable-default-apps",
      "--disable-extensions",
      "--disable-sync",
      "--hide-scrollbars",
      "--no-default-browser-check",
      "--no-first-run",
    ],
  });
  const page = await browser.newPage({ viewport: { width, height }, deviceScaleFactor: 1 });
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(250);
  await page.screenshot({ path: screenshotPath, fullPage: false });
  await browser.close();
  console.log(JSON.stringify({ status: "ready", screenshotPath, viewport: `${width}x${height}` }));
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
