## Requirements
- user can add a habit
  - when adding a habit, user can add attributes:
    - bad habit or good habit
    - days to commit
  - user can add a habit without inputting days to commit, so continuous
- user can remove a habit
  - when removing, user got a alert ui to make sure whether they want to remove the habit
- user can update habit status in a day
  - if eod user didn't update, it will be shown as missed
- when a habit is done for all requirements, user will got reward
- user can see habits they have
- user can see the status of every of habit
- user can log out


## UI Cases
- login frame
- welcome frame: list habits with the topmost is the least updated
  - can choose habits
- habit frame: nama habit, status bar, pertanyaan apakah sudah dilakukan hari ini atau belum ; habit yang sudah selesai tidak bisa diupdate

## components
### structs / class
- struct Habit
  - title str, start date, days i32, is_good bool, daily_status vec 1 dim
  - impl Add
  - impl Delete
- struct SQLite
  - impl Add, Delete, Update to a collection
- struct ListHabitWidget
  - impl Widget, receive list of Habits and build the list for it
- struct LoginPage
  - impl Page, render a login page and its logic
- struct HomePage
  - impl Page
- struct HabitPage
  - impl Page
- trait Page
  - render_page
  - back

### Specific Logic

#### Main
  - check if has been logged in
    - yes: continue
    - no: enter name
  - Welcome Page
    - when go into a habit page
      - turn off the rendering for the welcome page
      - render habit page
    - when return
      - do the same but for welcome page

