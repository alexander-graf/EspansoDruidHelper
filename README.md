# EspansoDruidHelper

##Espanso Helper: A User-Friendly Text Expansion Tool

Espanso can be a powerful tool for text expansion, but dealing with text files for configuration can be a bit daunting, especially for non-technical users. To make this process easier, I've developed a new program using the Rust language: the Espanso Helper.

Espanso Helper simplifies the creation of text replacements by providing a straightforward Graphical User Interface (GUI). Instead of manually editing text files, users can simply type in the 'trigger' and 'replacement' text in the GUI, and Espanso Helper does the rest.

Espanso Helper interacts with local files in a specific way. When you enter the 'trigger' and 'replacement' text in the GUI and click the 'Append' button, the program writes this information into a YAML file (kustom.yml) located in the .config/espanso/match directory within your home directory. This is where Espanso looks for text replacement definitions.

When appending to the file, Espanso Helper also ensures that the text is formatted correctly for Espanso. In Espanso, each text replacement rule must be specified in a distinct format. The 'trigger' is the text you type, and the 'replace' is the text that replaces the trigger. For single line replacements, the format is:

- trigger: "{trigger_text}"
  replace: "{replacement_text}"

For multi-line replacements, the format is slightly different:

- trigger: "{trigger_text}"
  replace: |
    {line1_of_replacement}
    {line2_of_replacement}
    ...

  The program automatically detects multi-line replacements and formats them correctly. It even handles the indentation required for multi-line replacements in YAML.

Finally, Espanso Helper also provides a handy 'Open Folder' button that opens the .config/espanso/match directory for you, making it easy to view or manually edit the kustom.yml file if needed.

In conclusion, by providing a user-friendly interface for creating text replacements, Espanso Helper makes it easier than ever to leverage the power of Espanso.
