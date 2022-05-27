# Anime List
## A command line tool built in Rust to serve as a local anime list for users
### Currently supports the following actions:
* add - Adds and anime to a file in json format
* export - Exports current list to a txt file with pretty printing
* list - Lists all current anime added to the terminal
* remove - Removes and anime from the list/file
* search - Searches for and prints the desired anime to terminal
* update - Update the fields of an existing anime 

### Usage
* Run the tool on the command line with the following format:<br>
``` anime-list [OPTIONS] <SUBCOMMAND>```
* Options:
    * -f <file_name>
        * Adding this lets the user choose the file name and location of the JSON file needed to store the list
        * If not supplied, the program will make a default file inside the users home directory
* Subcommands:
    * add
    * export
    * help
    * list
    * remove
    * search
    * update
* For example, in the following line: <br>
``` anime-list -f my_file.json add```
* The program would begin the process of adding a new entry to the json file supplied (in this case my_file.json)
* There is no extra items to supply to the command line, the program will ask the user via user input (stdin) for answers for relevant fields
* This is the same for all other subcommands