# sporepedia_getter
This gets a range of sporepedia creations from spore.com between two ID's.

# How to use
Install the latest release, unzip, and then run the exe.

It will open a command line window and prompt you to enter a starting creation ID and how many ID's you want to search. Once you give it both, it will collect all creations in that range (inclusive)
All creations will be put into a folder called "png_pile" in the same folder as wherever the executable is.

To terminate early just close the window.

# Miscellaneous important information
- It will take a bit to initialize when you first launch it. It's turning the ID lists in to a hashmap so it can easily search them, this is also where most of the memory usage comes from.
- There won't be any visual information displayed as it is gathering the pngs.
