## Installation Guide for Katalon Studio:

### Installing Katalon Studio Desktop App:

Download MacOSX latest version of Katalon Studio here: https://www.katalon.com/download/

Create an account and take note of your username and password: https://www.katalon.com/create-account/

Open dmg and activate Katalon Studio by choosing the Sign In option and using your username and password from katalon.com/

### Configurations for Web UI Testing:

Make sure that preferred web browsers are installed, refer to this guide for the list of supported browsers: 
https://docs.katalon.com/katalon-studio/docs/supported-environments.html

Download [Katalon Automation Recorder](https://chrome.google.com/webstore/detail/katalon-recorder/ljdobmomdgdljniojadhoplhkpialdid) extension for Chrome in order to capture screenshots in your Active Browser.

Katalon Automation Recorder can be accessed from your Chrome browser toolbar, however that will be covered later on.

When your build is activated, the Quick Guide screen is displayed to guide you through all major features. You can skip this and view the Quick Guide later from the Help menu.

### Create a Project: 

Click File > New > Project from the top Navigation 

Enter the name of your new project and the location to store the project data on your machine. Click OK.

### Inititate Your First Test Case Using Record Playback: 



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

Click Apply and OK

When your build is activated, the Quick Guide screen is displayed to guide you through all major features. You can skip this and view the Quick Guide later from the Help menu.




