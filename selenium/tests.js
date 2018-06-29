const chrome = require('selenium-webdriver/chrome');
const service = new chrome.ServiceBuilder('./deps/chromedriver').build();
chrome.setDefaultService(service);

const assert = require('assert');
const webdriver = require('selenium-webdriver');

describe('Google Search', () => {
  let driver;

  before(() => {
    driver = new webdriver.Builder()
      .withCapabilities(webdriver.Capabilities.chrome()).build();

    return driver.get('http://www.google.com');
  });

  after(() => {
    return driver.quit();
  })

  it('should work', async () => {
    const searchBox = await driver.findElement(webdriver.By.name('q'));
    await searchBox.sendKeys('simple programmer');

    return searchBox.getAttribute('value').then(function(value) {
      assert.equal(value, 'simple programmer');
    });
  });
});
