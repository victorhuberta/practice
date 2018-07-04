const chrome = require('selenium-webdriver/chrome');
const service = new chrome.ServiceBuilder('./deps/chromedriver').build();
chrome.setDefaultService(service);

const assert = require('assert');
const webdriver = require('selenium-webdriver');
const By = webdriver.By;

describe('Google Search', () => {
  let driver;

  before(() => {
    driver = new webdriver.Builder()
      .withCapabilities(webdriver.Capabilities.chrome()).build();
  });

  after(() => {
    return driver.quit();
  })

  beforeEach(() => {
    return driver.get('http://localhost:8000');
  });

  it('should open home and be redirected to buy page', async () => {
    const url = await driver.getCurrentUrl();
    assert(url.endsWith('/buy.html'));
  });

  it('should open buy page and see title', async () => {
    const buyLink = await driver.findElement(By.className('js-buy-link'));
    await buyLink.click();

    const title = await driver.getTitle();
    assert.equal(title, 'Buy');
  });

  it('should open about page and see title', async () => {
    const aboutLink = await driver.findElement(By.className('js-about-link'));
    await aboutLink.click();

    const title = await driver.getTitle();
    assert.equal(title, 'About');
  });

  it('should see items in shop', async () => {
    const shops = await driver.findElements(By.className('js-shop'));
    assert(shops.length > 0);

    const shopItems = await shops[0].findElements(By.className('js-shop-item'));
    assert(shopItems.length > 0);
  });
});
