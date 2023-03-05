# TFC Duties Bot 

Telegram bot created to help GAs of TFC do their duties. It is able to format parade state messages and search their duties for the month.

## Libraries / Hosting

* Created with the ```Teloxide``` library for Rust. Uses the dialogue system to get inputs from users.
* The bot is deployed as a docker container on fly.io.

## Functionalities

### Parade States
* Formats parade state messages sent from Whatsapp group.
* Turns hard-to-read message into a nicely formatted list.
* Uses ```regex``` and ```chrono``` libraries to parse, extract, and rearrange key information in the following format:
  * RANK | NAME
  * DETAILS

#### Usage
* User selects "PS", copies parade state message over from Whatsapp, then pastes the message in.
* ![Parade State demo](/media/PS.gif)

### Duties
* Searches the GA Duty Roster Google Sheet spreadsheet for a specific GAs liable duties.
* Retrieves key information of dates and duty name without the inconvenience of checking the spreadsheet itself.
* Uses ```reqwest``` and ```csv``` libraries to get the spreadsheet data and parse it in the CSV format. Information is presented as such:
  * DATE, DAY
  * ACTIVITY
  * OTHER GAs DOING ACTIVITY

#### Usage
* User selects "Duties", then answers name and month to query for in the Duty Roster spreadsheet.
* ![Duties demo](/media/Duties.gif)

## Future Work

* When users check for duties, show the assigned Duty Commander on the day that they have to perform OD.
* Parse the duties specified in the "OTHERS" column of the GA Duty Roster spreadsheet, and which duties the user is actually doing.






