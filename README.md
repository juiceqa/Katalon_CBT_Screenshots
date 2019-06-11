## PM Installation Guide for Katalon Studio and Katalon Recorder:

### Installing Katalon Studio Desktop App:

Download MacOSX latest version of Katalon Studio here: https://www.katalon.com/download/

Create an account [here](https://www.katalon.com/create-account/) and take note of your username and password.

Open dmg and activate Katalon Studio by choosing the Sign In option and using your username and password from katalon.com/

### Configurations for Web UI Testing:

Make sure that preferred web browsers are installed,[ refer to this guide](https://docs.katalon.com/katalon-studio/docs/supported-environments.html) for the list of supported browsers:

Download [Katalon Automation Recorder](https://chrome.google.com/webstore/detail/katalon-recorder/ljdobmomdgdljniojadhoplhkpialdid) extension for Chrome in order to capture screenshots in your Active Browser.

Katalon Automation Recorder can be accessed from your Chrome browser toolbar, however that will be covered later on.

When your build is activated, the Quick Guide screen is displayed to guide you through all major features. You can skip this and view the Quick Guide later from the Help menu.


### Create a New Project

Click *File > New > Project* from top navigation menu.

Use your the following naming convention for your new project:

JobID.ProjectName.Screenshots

Create a folder on Box in the same directory as your other project assets titled Screenshots *(Share Folder access with QA Lead for review and upload to this GitHub repository)*

Select your newly created folder from the Katalon Studio New Project Set-Up screen.


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

With your *Project Settings* window already open click add *External Libraries* and locate the *ashot-1.5.4.jar* file from your *Downloads* folder. 

Click *Apply* then *OKAY*.


### Environment Setup for Mobile Testing:

In case of Mobile testing, you need to install Node.js, Appium and enable USB Debugging mode on your device first.

Install macOS's missing pkg dependency handler, Homebrew, from your Terminal:

`/usr/bin/ruby -e "$(curl -fsSL 
https://raw.githubusercontent.com/Homebrew/install/master/install)"`

Install Carthage with Homebrew through Terminal: 

`brew install carthage`

Install both node and npm with Homebrew through Terminal:

`brew install node`
`brew install npm`

Install Appium with npm through Terminal: 

`npm install -g appium`

Open dmg and activate Katalon Studio using your username and password from katalon.com/

Click Katalon Studio from the top nav menu, go to *Preferences > Katalon > Mobile* and set the Appium directory to */usr/local/lib/node_modules/appium* 

Click *Apply* and *OK*

When your build is activated, the Quick Guide screen is displayed to guide you through all major features. Feel free to explore but for the purpose of this setup you can skip this and view the Quick Guide later from the Help menu.

