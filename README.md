# Developer Environment Setup:

### Homebrew, NPM and Node 

For Mobile testing, you will need to install Node.js, Appium.
      Also, be certain to enable USB Debugging mode on your device.

Checking and Installing Homebrew from your Terminal:
    Copy and paste this into your terminal:

            if [[ $(command -v brew) == "" ]]; then
                echo "Installing Hombrew"
                /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
            else
                echo "Updating Homebrew"
                brew update
            fi


Install Carthage in Terminal: 
    Copy and paste this into your terminal:
`brew install carthage`

Install both node and npm in Terminal:
    Copy and paste this into your terminal:

        `brew install node`
        `brew install npm`

### WebDriver Manager -- _Selenium server and browser driver manager_

From here Protractor's spinoff tool, `webdriver-manager` will take care of all of the rest of the heavy lifting for you.

You can download the npm package from [here](https://www.npmjs.com/package/webdriver-manager) or follow the instructions below from your Terminal.

To install run:

`install -g webdriver-manager`

Next, download the latest selenium server jar and chromedriver binary:
    Copy and paste this into your terminal:
        'webdriver-manager update'

Starting the Server
     Copy and paste this into your terminal:
     `webdriver-manager start`
     Navigate to: 
     http://localhost:4444/wd/hub.

If webdriver-manager start does not work, try to clear out the saved files.
    Copy and paste this into your terminal:
        'webdriver-manager clean'

Running / stopping server in background process (stopping is not yet supported on standalone server 3.x.x):
    Copy and paste this into your terminal:
        'webdriver-manager start --detach'
        'webdriver-manager shutdown'

### WebDriver Manager mobile setup:

 If you want to run tests on Android run `webdriver-manager update --android --android-accept-licenses`, this will set you up with the Android SDK, Appium _(Selenium wrapper for mobile browsers and apps)_ and sign all the Android license agreements for you.

For iOS run `webdriver-manager update --ios`, this checks your machine for iOS simulation capabilities. Due to Apple's strict set up, you'll also need to download the xcode commandline tools and agree to Apple's license agreement on your own. If you want to install more virtual devices than the xcode commandline tools offer, you can run `xcrun simctl` to help you with that.

### Configure Katalon Recorder to use Appium for Mobile Testing:

Open dmg and activate Katalon Studio using your username and password from katalon.com/

Click Katalon Studio from the top nav menu, go to *Preferences > Katalon > Mobile* and set the Appium directory to */usr/local/lib/node_modules/appium* 

Click *Apply* and *OK*

### Set Up CrossBrowserTesting Integration and Additional Project Settings:

Click *Project* in the main navigation and select *Settings* 

Expand the *Desired Capabilities* sidebar navigation menu item and it's subnav menu item WebUI

Select remote and enter the following into the *Remote web server url field:* http://qa%40juicepharma.com:u2168a770387f8d7@hub.crossbrowsertesting.com:80/wd/hub

Choose *Selenium* for the Remote web server type

Click Add and follow the grid below: 

| Name           | Type         | Value         |
| :------------- | :----------: | -----------:  |
| browserName    | String       | Chrome        |
| version        | String       | 74            |
| platform       | String       | Mac OSX 10.14 |   
| screenResolution | String     | 1920x1080     |
| record_video   | Boolean      | true          |
| name           | String       | your.project  |
| timeout        | String       | 30000         |
| handlesAlerts  | Boolean      | true          | 
| nativeEvents   | Boolean      | true          |
| takesScreenshot | Boolean     | true          |

Then click *Apply* 

Your Project Settings table should then programmatically add the following two lines: 

| Name               | Type         | Value         |
| :-------------     | :----------: | -----------:  |
| remoteWebDriverUrl | String       | http://qa%40juicepharma.com:u2168a770387f8d7@hub.crossbrowsertesting.com:80/wd/hub | 
| remoteWebDriverType | String      | Selenium      |

If you get lost you can refer to [CrossBrowserTesting's documentation on Katalon integration](https://help.crossbrowsertesting.com/integrations/tutorials/katalon-studio/). Or better yet, just ask Garrett, Lead QA, or your developer on the project for help. 


### Adding aShot Screenshot Utility to Enable Full Page Screenshots:

Download the jar file avaliable from this [Maven Repository](https://mvnrepository.com/artifact/ru.yandex.qatools.ashot/ashot/1.5.4).

With your *Project Settings* window already open click add *External Libraries* and locate the *ashot-1.5.4.jar* file from your *Downloads* folder. Click "Apply" and then "OK"
