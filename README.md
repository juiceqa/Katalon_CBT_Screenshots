## Installation Guide for Katalon Studio:

Download MacOSX latest version of Katalon Studio here: https://www.katalon.com/download/

Create an account and take note of your username and password: https://www.katalon.com/create-account/

Open dmg and activate Katalon Studio using your username and password from katalon.com/

If you do Web UI testing, then no additional settings are needed except making sure the required browsers are installed. Check this list for all supported browsers:

https://docs.katalon.com/katalon-studio/docs/supported-environments.html

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

Screenshot



