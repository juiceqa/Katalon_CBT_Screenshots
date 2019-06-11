# PM Setup Guide for Katalon Studio and Katalon Recorder:


## Objective: 

TBD - need to update


## Collaborators: 

Eric Norcross, VP of Technology
Garrett Glick, QA Lead
Jozef Woroniecki, Engineer


### Installing Katalon Studio Desktop App:

Download MacOSX latest version of [Katalon Studio](https://www.katalon.com/download/)

[Create an account](https://www.katalon.com/create-account/) and take note of your username and password.

Open the dmg file that in your Downloads folder and activate Katalon Studio by choosing the Sign-In option and using credentials from above.


### Katalon Recorder Download:

Download [Katalon Automation Recorder](https://chrome.google.com/webstore/detail/katalon-recorder/ljdobmomdgdljniojadhoplhkpialdid) extension for Chrome in order to capture screenshots in your Active Chrome Browser.

You should see the Katalon Automation Recorder icon in the top left corner of your Chrome browser toolbar

_if you don't see it:_

Click the three dots in the upper left corner of your browser and select More Tools > Extensions. Find Katalon Recorder and make sure it's toggled on.

### Create a New Project in Katalon Studio

Click *File > New > Project* from top navigation menu.

Use your the following naming convention for your new project:

JobID ProjectName Screenshots

Create a folder on Box in the same directory as your other project assets titled Screenshots *(Share Folder access with QA Lead for review and upload to this GitHub repository)*

Select your newly created folder from the Katalon Studio New Project Set-Up screen.

### Import Project Settings Template:

Click the Green *"Download or Clone"* button at the top of this page and select *"Download Zip"*

Unzip the file and put the Alkermes-United-For-Schizophrenia-Website folder on your Desktop.

Re-open Katalon Studio and Click File > Import Settings from the top nav

Select the Alkermes-United-For-Schizophrenia-Website folder and click *OKAY.*

... 

# Developer Environment Setup:

### Homebrew, NPM and Node 

In case of Mobile testing, you need to install Node.js, Appium and enable USB Debugging mode on your device first.

Install macOS's missing pkg dependency handler, Homebrew, from your Terminal:

`/usr/bin/ruby -e "$(curl -fsSL 
https://raw.githubusercontent.com/Homebrew/install/master/install)"`

Install Carthage with Homebrew through Terminal: 

`brew install carthage`

Install both node and npm with Homebrew through Terminal:

`brew install node`
`brew install npm`

### WebDriver Manager -- _Selenium server and browser driver manager_

From here Protractor's spinoff tool, `webdriver-manager` will take care of all of the rest of the heavy lifting for you.

You can download the npm package from [here](https://www.npmjs.com/package/webdriver-manager) or follow the instructions below from your Terminal.

To install run:

`nstall -g webdriver-manager`

Then you'll need to download the latest selenium server jar and chromedriver binary. Simple as:

`webdriver-manager start`

### WebDriver Manager mobile setup:

All set for desktop. If you want to run tests on Android run `webdriver-manager update --android --android-accept-licenses`, this will set you up with the Android SDK, Appium _(Selenium wrapper for mobile browsers and apps)_ and sign all the Android license agreements for you.

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

With your *Project Settings* window already open click add *External Libraries* and locate the *ashot-1.5.4.jar* file from your *Downloads* folder. 

Click *Apply* then *OKAY*.

