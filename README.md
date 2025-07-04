# VSCode Launcher

VSCode Launcher is a simple CLI tool that helps to run vscode configuration without opening it avoiding the resource usage derived from extensions and vscode itself.

## Pain in the ass

VSCode `launch.json` is very useful when we want to define and store run configurations with other people in our team.
However, when I just want to run a specific configuration I need to start vscode and getting all the overhead in resource consumption.

An obvious solution is to run directly the Python application from terminal but I need to copy all the environment variables from the `launch.json` configuration to a `.env` file and then using `python-dotenv` library with `load_dotenv()` method to load them.

This is not a scalable solution because I need to remember every time I run the command to align the .env file that is in the `.gitignore` and therefore, not versioned.

## Solution

So, I decided to leave the environment variables and all the other configuration directly inside the `launch.json` file and use a CLI tool to parse it and run the proper configuration.

Yes, I'm over engineering all of this BUT it is just an excuse to develop a CLI tool with Rust ;)
