import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths

import javax.imageio.ImageIO

import org.openqa.selenium.WebDriver

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import ru.yandex.qatools.ashot.AShot
import ru.yandex.qatools.ashot.Screenshot
import ru.yandex.qatools.ashot.coordinates.WebDriverCoordsProvider
import ru.yandex.qatools.ashot.shooting.ShootingStrategies

// take screenshot of entire web page
void takeEntirePage(WebDriver webDriver, File file, Integer timeout = 300) {
	Screenshot screenshot = new AShot().
			coordsProvider(new WebDriverCoordsProvider()).
			shootingStrategy(ShootingStrategies.viewportPasting(timeout)).
			takeScreenshot(webDriver)
	ImageIO.write(screenshot.getImage(), "PNG", file)
}

WebUI.openBrowser('')

// set display size to largest available on CrossBrowserTesting.com
WebUI.setViewPortSize(1920, 1200)

def topPageUrl = 'https://alkermes-dse.azurewebsites.net/share-your-story'

WebUI.navigateToUrl(topPageUrl)

// this verification will PASS
WebUI.verifyElementPresent(
	findTestObject('Object Repository/Page_Share Your Story Form/button_Submit Your Story'),
	10,
	FailureHandling.CONTINUE_ON_FAILURE)

// take screenshot and save a PNG file into Reports dir
Path projectDir = Paths.get(RunConfiguration.getProjectDir())
Path reportDir = projectDir.resolve('Screenshots')
Files.createDirectories(reportDir)
Path pngFile = reportDir.resolve('ShareYourStory.png')
WebDriver driver = DriverFactory.getWebDriver()

//WebUI.takeScreenshot(pngFile.toString(), FailureHandling.STOP_ON_FAILURE)
takeEntirePage(driver, pngFile.toFile(), 500)

WebUI.closeBrowser()
