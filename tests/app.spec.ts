import { test, expect } from "@playwright/test";

test.describe("Handy App", () => {
  test("dev server responds", async ({ page }) => {
    const response = await page.goto("/");
    expect(response?.status()).toBe(200);
  });

  test("page has html structure", async ({ page }) => {
    await page.goto("/");

    const html = await page.content();
    expect(html).toContain("<html");
    expect(html).toContain("<body");
  });

  test("root element is rendered", async ({ page }) => {
    await page.goto("/");
    const root = page.locator("#root");
    await expect(root).toBeAttached();
  });

  test("page title is set", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle("handy");
  });

  test("app loads React content into root", async ({ page }) => {
    await page.goto("/");
    // React should mount something inside #root
    const root = page.locator("#root");
    // Wait briefly for React to hydrate
    await page.waitForTimeout(1000);
    const childCount = await root.evaluate((el) => el.children.length);
    expect(childCount).toBeGreaterThan(0);
  });
});
