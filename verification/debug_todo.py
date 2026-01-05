from playwright.sync_api import sync_playwright

def run(playwright):
    browser = playwright.chromium.launch()
    page = browser.new_page()
    page.on("console", lambda msg: print(f"CONSOLE: {msg.text}"))
    page.on("pageerror", lambda exc: print(f"PAGE ERROR: {exc}"))
    try:
        page.goto("http://localhost:5173")
        page.wait_for_timeout(3000)
    except Exception as e:
        print(f"NAV ERROR: {e}")
    page.screenshot(path="verification/todolist.png")
    browser.close()

with sync_playwright() as playwright:
    run(playwright)
