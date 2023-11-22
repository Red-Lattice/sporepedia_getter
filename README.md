# sporepedia_getter
This gets a range of sporepedia creations from spore.com starting at specified ID and ending at some amount of ID's after.

# How to use
Install the latest release, unzip, and then run the exe.

It will open a command line window and prompt you to enter a starting creation ID and how many ID's you want to search. Once you give it both, it will collect all creations in that range (inclusive)
All creations will be put into a folder called "png_pile" in the same directory as wherever the executable is.

# Miscellaneous important information
- It will take a bit to initialize when you first launch it. It's turning the ID lists in to a hashmap so it can easily search them, this is also where most of the memory usage comes from.
- There won't be any visual information displayed as it is gathering the pngs.

# ID List Information
All ID's are stored in a folder called id_stack. This list may be incomplete. In order to update it, just add a text file, or folders containing text files, somewhere in the id_stack folder. On starting up the program it will automatically hash all ID's it finds in the folder.
