# Filend [WORK IN PROGRESS]

A utility program aiming to make it convenient to directly share files with people in your LAN/VPN.

## Project completion

See the current completion status, [here](PROJECT_PLAN.md).

## Usage

The program requires you and your friend to have direct IP connection between your machines (either in your LAN or through VPN like Hamachi, RadminVPN or ZeroTier-One).

### Sender

1. Add a file to the list of shared files
2. Copy the URL from the app
3. Send it to your friend
4. Wait until your friend receives the file

### Receiver

1. Open the URL

## How it works

The program launches a HTTP server on a custom port (default: 7789), based on Actix-Web.
Direct connection between your computers makes it possible to generate unique URLs to identify files that you share between each other
