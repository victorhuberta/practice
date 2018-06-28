var webdriver = require('selenium-webdriver');

var driver = new webdriver.Builder()
  .forBrowser('chrome')
  .usingServer('http://localhost:4444/wd/hub')
  .build();

driver.get('http://www.google.com');
driver.findElement(webdriver.By.name('q')).sendKeys('simple programmer');
driver.findElement(webdriver.By.name('btnG')).click();
//driver.quit();
