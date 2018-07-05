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

    const title = await driver.getTitle();
    assert.equal(title, 'Buy');
  });

  it('should open buy page and see title', async () => {
    const buyLink = await driver.findElement(By.className('js-buy-link'));
    await buyLink.click();

    const title = await driver.getTitle();
    assert.equal(title, 'Buy');
  });

  it('should open help page and see title', async () => {
    const helpLink = await driver.findElement(By.className('js-help-link'));
    await helpLink.click();

    const title = await driver.getTitle();
    assert.equal(title, 'Help');
  });

  it('should see all items in shop', async () => {
    const shops = await driver.findElements(By.className('js-shop'));
    assert.equal(shops.length, 1);

    const options = await shops[0].findElements(By.css('.js-shop .thumbnail'));
    assert.equal(options.length, 7);
  });

  it('should select multiple items in shop', async () => {
    const options = await driver.findElements(By.css('.js-shop .thumbnail'));
    await options[0].click();
    await options[1].click();

    const selectedOptions = await driver.findElements(By.css('.js-shop .thumbnail.selected'));
    assert.equal(selectedOptions.length, 2);
  });

  it('should add multiple items to cart', async () => {
    // Select multiple items to be added to cart.
    const shopOptions = await driver.findElements(By.css('.js-shop .thumbnail'));
    await shopOptions[0].click();
    await shopOptions[1].click();

    // Add items to cart.
    const addToCart = await driver.findElement(By.className('js-add-to-cart'));
    await addToCart.click();

    // Check if the selected items have been added into the cart.
    const cartOptions = await driver.findElements(By.css('.js-cart .thumbnail'));
    assert.equal(cartOptions.length, 2);
  });

  it('should remove multiple items from cart', async () => {
    // Select multiple items to be added to cart.
    let shopOptions = await driver.findElements(By.css('.js-shop .thumbnail'));
    shopOptions.forEach(async option => {
      await option.click();
    });
    // Add items to cart.
    const addToCart = await driver.findElement(By.className('js-add-to-cart'));
    await addToCart.click();

    // Select multiple items to be removed from cart.
    const cartOptions = await driver.findElements(By.css('.js-cart .thumbnail'));
    await cartOptions[0].click();
    await cartOptions[1].click();
    // Remove items from cart.
    const removeFromCart = await driver.findElement(By.className('js-remove-from-cart'));
    await removeFromCart.click();

    // Check if the selected items have been added back into the shop.
    shopOptions = await driver.findElements(By.css('.js-shop .thumbnail'));
    assert.equal(shopOptions.length, 2);
  });

  it('should checkout the cart', async () => {
    const checkout = await driver.findElement(By.className('js-checkout'));
    await checkout.click();

    const title = await driver.getTitle();
    assert.equal(title, 'Confirm Order');
  });

  it('should fill personal data and confirm order', async () => {
    await driver.get('http://localhost:8000/confirm-order.html');

    // Fill form data.
    await driver.findElement(By.className('js-person-name')).sendKeys('Foo Bar');
    await driver.findElement(By.className('js-person-phone')).sendKeys('+62816555778889');
    await driver.findElement(By.className('js-person-address')).sendKeys('Jl. Foo Bar, Foor, Baro');
    // Confirm order.
    await driver.findElement(By.className('js-confirm-order')).click();

    // Check if an alert exists.
    return driver.switchTo().alert().then(
      () => {
        return driver.switchTo().alert().accept();
      },
      () => {
        assert(false);
      }
    );
  });

  it('should NOT confirm order if any of the personal data is missing', async () => {
    await driver.get('http://localhost:8000/confirm-order.html');

    // Fill form data.
    await driver.findElement(By.className('js-person-name')).sendKeys('Foo Bar');
    await driver.findElement(By.className('js-person-address')).sendKeys('Jl. Foo Bar, Foor, Baro');
    // Try to confirm order.
    await driver.findElement(By.className('js-confirm-order')).click();

    // Nothing happened.
    return driver.switchTo().alert().then(() => assert(false), () => {});
  });

  it('should fill help data and submit request', async () => {
    await driver.get('http://localhost:8000/help.html');

    // Fill form data.
    await driver.findElement(By.className('js-person-name')).sendKeys('Foo Bar');
    await driver.findElement(By.className('js-person-email')).sendKeys('hello@example.com');
    await driver.findElement(By.className('js-help-problem')).sendKeys('The cards are too shiny');
    // Confirm order.
    await driver.findElement(By.className('js-submit-request')).click();

    // Check if an alert exists.
    return driver.switchTo().alert().then(
      () => {
        return driver.switchTo().alert().accept();
      },
      () => {
        assert(false);
      }
    );
  });

  it('should NOT submit request if any of the help data is missing', async () => {
    await driver.get('http://localhost:8000/help.html');

    // Fill form data.
    await driver.findElement(By.className('js-person-name')).sendKeys('Foo Bar');
    await driver.findElement(By.className('js-person-email')).sendKeys('hello@example.com');
    // Try to confirm order.
    await driver.findElement(By.className('js-submit-request')).click();

    // Nothing happened.
    return driver.switchTo().alert().then(() => assert(false), () => {});
  });
});
