// This program allows users to securely manage files by backing them up,  
  restoring from backups, and deleting files. 

Configuration: To set the environment for testing, it is necessary to follow the next steps:  

1. Install Visual Studio Code and add rust extensions there.  
2. Open the PowerShell terminal in VS code and access to the working path.  
3. Run the code: cargo new my_project_name (that will build the project and dependencies).  
4. To run the code, we need to add the command chrono = "0.4" in the cargo.toml > dependencies (uder dependencies).  
5. Run the command cargo build (to build the project) 
6. Go to > main.rs, delete the existing code and create the code or paste the rust code (GitHub or provided in the report).  
7. Save, then Run the code, we can use commands individually or together like a single command:  
 
cls; cargo run; get-content logfile.txt 
Cls: clear the terminal 
Cargo run: Run the project 

8. Get-content logfile.txt: show the content of the logfile.txt to visualize changes (if using PowerShell terminal).  
9. After run Cargo run command, the program will ask the File_Name.ext you want to work with, the action you want to take and confirmation of action (for deleting), then, It will indicate actions taken.  

**Make sure write the correct name and extension of the file, as well as files exist in the actual working path.  
