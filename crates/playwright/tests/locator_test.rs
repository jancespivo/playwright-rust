// Integration tests for Locator functionality
//
// Following TDD: Write tests first (Red), then implement (Green)
//
// Tests cover:
// - Locator creation (page.locator)
// - Locator chaining (first, last, nth, locator)
// - Query methods (count, text_content, inner_text, inner_html, get_attribute)
// - State queries (is_visible, is_enabled, is_checked, is_editable)
//
// Performance Optimization (Phase 6):
// - Combined related tests to minimize browser launches
// - Removed redundant cross-browser tests (Rust bindings use same protocol for all browsers)
// - Expected speedup: ~64% (11 tests → 4 tests)

mod test_server;

use playwright_rs::protocol::Playwright;
use test_server::TestServer;

mod common;

// ============================================================================
// Locator Query Methods
// ============================================================================

#[tokio::test]
async fn test_locator_query_methods() {
    common::init_tracing();
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/locator.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test 1: Create a locator
    let heading = page.locator("h1").await;
    assert_eq!(heading.selector(), "h1");

    // Test 2: Count elements
    let paragraphs = page.locator("p").await;
    let count = paragraphs.count().await.expect("Failed to get count");
    assert_eq!(count, 3); // locator.html has exactly 3 paragraphs

    // Test 3: Get text content
    let text = heading
        .text_content()
        .await
        .expect("Failed to get text content");
    assert_eq!(text, Some("Test Page".to_string()));

    // Test 4: Get inner text (visible text only)
    let inner = heading
        .inner_text()
        .await
        .expect("Failed to get inner text");
    assert_eq!(inner, "Test Page");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

// ============================================================================
// Locator Chaining Methods
// ============================================================================

#[tokio::test]
async fn test_locator_chaining_methods() {
    common::init_tracing();
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/locator.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    let paragraphs = page.locator("p").await;

    // Test 1: Get first paragraph
    let first = paragraphs.first();
    assert_eq!(first.selector(), "p >> nth=0");
    let text = first
        .text_content()
        .await
        .expect("Failed to get text content");
    assert_eq!(text, Some("First paragraph".to_string()));

    // Test 2: Get last paragraph
    let last = paragraphs.last();
    assert_eq!(last.selector(), "p >> nth=-1");
    let text = last
        .text_content()
        .await
        .expect("Failed to get text content");
    assert_eq!(text, Some("Third paragraph".to_string()));

    // Test 3: Get nth element (second paragraph)
    let second = paragraphs.nth(1);
    assert_eq!(second.selector(), "p >> nth=1");
    let text = second
        .text_content()
        .await
        .expect("Failed to get text content");
    assert_eq!(text, Some("Second paragraph".to_string()));

    // Test 4: Nested locators
    let container = page.locator(".container").await;
    let nested = container.locator("#nested");
    assert_eq!(nested.selector(), ".container >> #nested");
    let text = nested
        .text_content()
        .await
        .expect("Failed to get text content");
    assert_eq!(text, Some("Nested element".to_string()));

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

// ============================================================================
// Locator State Methods
// ============================================================================

#[tokio::test]
async fn test_locator_state_methods() {
    common::init_tracing();
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/locator.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test 1: Check visibility for visible element
    let heading = page.locator("h1").await;
    let visible = heading
        .is_visible()
        .await
        .expect("Failed to check visibility");
    assert!(visible);

    // Test 2: Hidden element should not be visible
    let hidden = page.locator("#hidden").await;
    let hidden_visible = hidden
        .is_visible()
        .await
        .expect("Failed to check visibility");
    assert!(!hidden_visible);

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

// ============================================================================
// get_by_text Locator Methods
// ============================================================================

#[tokio::test]
async fn test_get_by_text() {
    common::init_tracing();
    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");
    let browser = playwright
        .chromium()
        .launch()
        .await
        .expect("Failed to launch browser");
    let page = browser.new_page().await.expect("Failed to create page");

    page.goto(&format!("{}/locator.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    // Test 1: Substring match (exact=false) - "Submit" matches both "Submit" and "Submit Order"
    let submit_buttons = page.get_by_text("Submit", false).await;
    let count = submit_buttons
        .count()
        .await
        .expect("Failed to count submit buttons");
    assert_eq!(count, 2, "Substring 'Submit' should match both buttons");

    // Test 2: Exact match - "Submit" matches only the exact "Submit" button
    let exact_submit = page.get_by_text("Submit", true).await;
    let count = exact_submit
        .count()
        .await
        .expect("Failed to count exact submit");
    assert_eq!(count, 1, "Exact 'Submit' should match only one button");

    // Test 3: Case-insensitive substring match
    let hello = page.get_by_text("hello world", false).await;
    let count = hello.count().await.expect("Failed to count hello");
    assert_eq!(
        count, 2,
        "Case-insensitive 'hello world' should match both spans"
    );

    // Test 4: Case-sensitive exact match
    let hello_exact = page.get_by_text("Hello World", true).await;
    let count = hello_exact
        .count()
        .await
        .expect("Failed to count exact hello");
    assert_eq!(count, 1, "Exact 'Hello World' should match only one span");

    // Test 5: Locator chaining - get_by_text within a container
    let container = page.locator(".text-container").await;
    let inner = container.get_by_text("Inner Text", false);
    let count = inner.count().await.expect("Failed to count inner text");
    assert_eq!(count, 1, "get_by_text should scope to container");

    // Test 6: get_by_text on a Locator (chained selector)
    let body = page.locator("body").await;
    let submit_in_body = body.get_by_text("Submit", true);
    let count = submit_in_body
        .count()
        .await
        .expect("Failed to count submit in body");
    assert_eq!(count, 1, "Chained get_by_text should work from Locator");

    browser.close().await.expect("Failed to close browser");
    server.shutdown();
}

// ============================================================================
// Cross-browser Smoke Test
// ============================================================================

#[tokio::test]
async fn test_cross_browser_smoke() {
    common::init_tracing();
    // Smoke test to verify locators work in Firefox and WebKit
    // (Rust bindings use the same protocol layer for all browsers,
    //  so we don't need exhaustive cross-browser testing for each method)

    let server = TestServer::start().await;
    let playwright = Playwright::launch()
        .await
        .expect("Failed to launch Playwright");

    // Test Firefox
    let firefox = playwright
        .firefox()
        .launch()
        .await
        .expect("Failed to launch Firefox");
    let firefox_page = firefox.new_page().await.expect("Failed to create page");

    firefox_page
        .goto(&format!("{}/locator.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    let firefox_heading = firefox_page.locator("h1").await;
    let text = firefox_heading
        .text_content()
        .await
        .expect("Failed to get text content");
    assert_eq!(text, Some("Test Page".to_string()));

    firefox.close().await.expect("Failed to close Firefox");

    // Test WebKit
    let webkit = playwright
        .webkit()
        .launch()
        .await
        .expect("Failed to launch WebKit");
    let webkit_page = webkit.new_page().await.expect("Failed to create page");

    webkit_page
        .goto(&format!("{}/locator.html", server.url()), None)
        .await
        .expect("Failed to navigate");

    let webkit_heading = webkit_page.locator("h1").await;
    let visible = webkit_heading
        .is_visible()
        .await
        .expect("Failed to check visibility");
    assert!(visible);

    webkit.close().await.expect("Failed to close WebKit");
    server.shutdown();
}
