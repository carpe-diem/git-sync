# Notes Sync

A simple application to synchronize your notes with GitHub. Keep your notes backed up and accessible across multiple devices using Git as the synchronization mechanism.

## Features
    ...

## TODO
-[ ] Automatic synchronization with GitHub repositories
-[ ] Support for multiple GitHub repositories
-[ ] Two-way sync between local files and remote repository

## Description

Notes Sync is a tool designed to help you maintain your notes organized and synchronized using GitHub as a backend. It watches for changes in your local notes directory and automatically commits and pushes changes to your configured GitHub repository.

## Usage
    TODO

## Installation
    TODO

## Configuration
    TODO

## Project Structure

```
notes-sync
├── src
│   ├── main.rs          // Application entry point
│   ├── config.rs        // Project configuration (GitHub token, paths, etc.)
│   ├── github.rs        // Module to interact with the GitHub API
│   ├── notes.rs         // Module to read macOS notes
│   ├── sync.rs          // Synchronization logic
├── Cargo.toml           // Project dependencies and configuration
├── README.md            // Project documentation

```