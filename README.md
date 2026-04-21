# Studlyn 
Using diouxus to create the GUI for the desktop app

### On going features:
- Creating session state to store pauses and breaks during the study session,(thinking of using json to tempo session data, but not sure yet)
- After finalizing the session data inserted on the db, the user behavior tracking will start  

### Implemented:
- Basic Start, Stop and Break between clock modes
- DB start into user pc directory with rusqlite(Sqlite db), *User session table*
- Session Query logic working, but not plugged into clock already
- GUI for clocks (Adding more personalization and theme modes)

### Current GUI of the countdown timer (20/04)
<img width="983" height="352" alt="image" src="https://github.com/user-attachments/assets/7a8e32f9-9a27-4426-9446-5a9803341869" />
