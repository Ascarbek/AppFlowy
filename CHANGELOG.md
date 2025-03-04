# Release Notes

## Version 0.2.4 - 06/23/2023

### Bug fixes:

- Unable to copy and paste a word
- Some UI issues

## Version 0.2.3 - 06/21/2023

### New Features

- Added support for creating multiple database views for existing database

## Version 0.2.2 - 06/15/2023

### New Features

- Added support for embedding a document in the database's row detail page
- Added support for inserting an emoji in the database's row detail page

### Other Updates

- Added language selector on the welcome page
- Added support for importing multiple markdown files all at once

## Version 0.2.1 - 06/11/2023

### New Features

- Added support for creating or referencing a calendar in the document
- Added `+` icon in grid's add field

### Other Updates

- Added vertical padding for progress bar
- Hide url cell accessory when the content is empty

### Bug fixes:

- Fixed unable to export markdown
- Fixed adding vertical padding for progress bar
- Fixed database view didn't update after the database layout changed.

## Version 0.2.0 - 06/08/2023

### New Features

- Improved checklists to support each cell having its own list
- Drag and drop calendar events
- Switch layouts (calendar, grid, kanban) of a database
- New database properties: 'Updated At' and 'Created At'
- Enabled hiding properties on the row detail page
- Added support for reordering and saving row order in different database views.
- Enabled each database view to have its own settings, including filter and sort options
- Added support to convert `“` (double quote) into a block quote
- Added support to convert `***` (three stars) into a divider
- Added support for an 'Add' button to insert a paragraph in a document and display the slash menu
- Added support for an 'Option' button to delete, duplicate, and customize block actions

### Other Updates

- Added support for importing v0.1.x documents and databases
- Added support for database import and export to CSV
- Optimized scroll behavior in documents.
- Redesigned the launch page

### Bug fixes

- Fixed bugs related to numbers
- Fixed issues with referenced databases in documents
- Fixed menu overflow issues in documents

### Data migration

The data format of this version is not compatible with previous versions. Therefore, to migrate your data to the new version, you need to use the export and import functions. Please follow the guide to learn how to export and import your data.

#### Export files in v0.1.6

https://github.com/AppFlowy-IO/AppFlowy/assets/11863087/0c89bf2b-cd97-4a7b-b627-59df8d2967d9

#### Import files in v0.2.0

https://github.com/AppFlowy-IO/AppFlowy/assets/11863087/7b392f35-4972-497a-8a7f-f38efced32e2

## Version 0.1.5 - 11/05/2023

### Bug Fixes

- Fix: calendar dates don't match with weekdays.
- Fix: sort numbers in Grid.

## Version 0.1.4 - 04/05/2023

### New features

- Use AppFlowy’s calendar views to plan and manage tasks and deadlines.
- Writing can be improved with the help of OpenAI.

## Version 0.1.3 - 24/04/2023

### New features

- Launch the official Dark Mode.
- Customize the font color and highlight color by setting a hex color value and an opacity level.

### Bug Fixes

- Fix: the slash menu can be triggered by all other keyboards than English.
- Fix: convert the single asterisk to italic text and the double asterisks to bold text.

## Version 0.1.2 - 03/28/2023

### Bug Fixes

- Fix: update calendar selected range.
- Fix: duplicate view.

## Version 0.1.1 - 03/21/2023

### New features

- AppFlowy brings the power of OpenAI into your AppFlowy pages. Ask AI to write anything for you in AppFlowy.
- Support adding a cover image to your page, making your pages beautiful.
- More shortcuts become available. Click on '?' at the bottom right to access our shortcut guide.

### Bug Fixes

- Fix some bugs

## Version 0.1.0 - 02/09/2023

### New features

- Support linking a Board or Grid into the Document page
- Integrate a callout plugin implemented by community members
- Optimize user interface

### Bug Fixes

- Fix some bugs

## Version 0.0.9.1 - 01/03/2023

### New features

- New theme
- Support changing text color of editor
- Optimize user interface

### Bug Fixes

- Fix some grid bugs

## Version 0.0.9 - 12/21/2022

### New features

- Enable the user to define where to store their data
- Support inserting Emojis through the slash command

### Bug Fixes

- Fix some bugs

## Version 0.0.8.1 - 12/09/2022

### New features

- Support following your default system theme
- Improve the filter in Grid

### Bug Fixes

- Copy/Paste

## Version 0.0.8 - 11/30/2022

### New features

- Table-view database
  - support column type: Checklist
- Board-view database
  - support column type: Checklist
- Customize font size: small, medium, large

## Version 0.0.7.1 - 11/30/2022

### Bug Fixes

- Fix some bugs

## Version 0.0.7 - 11/27/2022

### New features

- Support adding filters by the text/checkbox/single-select property in Grid

## Version 0.0.6.2 - 10/30/2022

- Fix some bugs

## Version 0.0.6.1 - 10/26/2022

### New features

- Optimize appflowy_editor dark mode style

### Bug Fixes

- Unable to copy the text with checkbox or link style

## Version 0.0.6 - 10/23/2022

### New features

- Integrate **appflowy_editor**

## Version 0.0.5.3 - 09/26/2022

### New features

- Open the next page automatically after deleting the current page
- Refresh the Kanban board after altering a property type

### Bug Fixes

- Fix switch board bug
- Fix delete the Kanban board's row error
- Remove duplicate time format
- Fix can't delete field in property edit panel
- Adjust some display UI issues

## Version 0.0.5.2 - 09/16/2022

### New features

- Enable adding a new card to the "No Status" group
- Fix some bugs

### Bug Fixes

- Fix cannot open AppFlowy error
- Fix delete the Kanban board's row error

## Version 0.0.5.1 - 09/14/2022

### New features

- Enable deleting a field in board
- Fix some bugs

## Version 0.0.5 - 09/08/2022

### New features - Kanban Board like Notion and Trello beta

Boards are the best way to manage projects & tasks. Use them to group your databases by select, multiselect, and checkbox.

<p align="left"><img src="https://user-images.githubusercontent.com/12026239/190055984-6efa2d7a-ee38-4551-859e-ee56388e1859.gif" width="1000px" /></p>

- Set up columns that represent a specific phase of the project cycle and use cards to represent each project/task
- Drag and drop a card from one phase/column to another phase/column
- Update database properties in the Board view by clicking on a property and making edits on the card

### Other Features & Improvements

- Settings allow users to change avatars
- Click and drag the right edge to resize your sidebar
- And many user interface improvements (link)

## Version 0.0.5 - beta.2 - beta.1 - 09/01/2022

### New features

- Board-view database
  - Support start editing after creating a new card
  - Support editing the card directly by clicking the edit button
  - Add the `No Status` column to display the cards while their status is empty

### Bug Fixes

- Optimize insert card animation
- Fix some UI bugs

## Version 0.0.5 - beta.1 - 08/25/2022

### New features

- Board-view database
  - Group by single select
  - drag and drop cards
  - insert / delete cards

![Aug-25-2022 16-22-38](https://user-images.githubusercontent.com/86001920/186614248-23186dfe-410e-427a-8cc6-865b1f79e074.gif)

## Version 0.0.4 - 06/06/2022

- Drag to adjust the width of a column
- Upgrade to Flutter 3.0
- Native support for M1 chip
- Date supports time formats
- New property: URL
- Keyboard shortcuts support for Grid: press Enter to leave the edit mode; control c/v to copy-paste cell values

### Bug Fixes

- Fixed some bugs

## Version 0.0.4 - beta.3 - 05/02/2022

- Drag to reorder app/ view/ field
- Row record opens as a page
- Auto resize the height of the row in the grid
- Support more number formats
- Search column options, supporting Single-select, Multi-select, and number format

![May-03-2022 10-03-00](https://user-images.githubusercontent.com/86001920/166394640-a8f1f3bc-5f20-4033-93e9-16bc308d7005.gif)

### Bug Fixes & Improvements

- Improved row/cell data cache
- Fixed some bugs

## Version 0.0.4 - beta.2 - 04/11/2022

- Support properties: Text, Number, Date, Checkbox, Select, Multi-select
- Insert / delete rows
- Add / delete / hide columns
- Edit property
  ![](https://user-images.githubusercontent.com/12026239/162753644-bf2f4e7a-2367-4d48-87e6-35e244e83a5b.png)

## Version 0.0.4 - beta.1 - 04/08/2022

v0.0.4 - beta.1 is pre-release

### New features

- Table-view database
  - support column types: Text, Checkbox, Single-select, Multi-select, Numbers
  - hide / delete columns
  - insert rows

## Version 0.0.3 - 02/23/2022

v0.0.3 is production ready, available on Linux, macOS, and Windows

### New features

- Dark Mode
- Support new languages: French, Italian, Russian, Simplified Chinese, Spanish
- Add Settings: Toggle on Dark Mode; Select a language
- Show device info
- Add tooltip on the toolbar icons

Bug fixes and improvements

- Increased height of action
- CPU performance issue
- Fix potential data parser error
- More foundation work for online collaboration
