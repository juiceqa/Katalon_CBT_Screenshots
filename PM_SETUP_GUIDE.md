# PM Setup Guide for Katalon Studio and Katalon Recorder:


## Objective: 

Automate full screenshot and unit screen shot testing.


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

From top navigation menu click *File > New > Project* 

Use your the following naming convention for your new project:

JobID ProjectName Screenshots
[Example: B26354 TheNewIngrezzaSite Screenshots]

Create a folder on Box in the same directory as your other project assets titled Screenshots *(Share Folder access with QA Lead for review and upload to this GitHub repository)*
--- This would require them to know how to use git or have access to github
--or-- should we include instructions how to do that

Select your newly created folder from the Katalon Studio New Project Set-Up screen.
-- show navigation to this from within katalon.  Is this the Box folder from previous instruction or from the top navigation?

### Import Project Settings Template:

Click the Green *"Download or Clone"* button at the top of this page and select *"Download Zip"*

Unzip the file and put the Alkermes-United-For-Schizophrenia-Website folder on your Desktop.

Re-open Katalon Studio and Click File > Import Settings from the top nav
--this did not work for me on mac.  
--the proper path for me was: desktop > katalon_file > sample project > Alkermes_united... > include > config 

Select the Alkermes-United-For-Schizophrenia-Website folder and click *OKAY.*

... _DEVS CONTINUE ONTO DEV_README.md for further set up_
-- you renamed this README.md but indicated, above, the file was called DEV_README.....DEV_README is proper

### Record Test Case 


