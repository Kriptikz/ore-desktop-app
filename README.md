# Unofficial Ore Mining Desktop App

- OreV2 is under active development. The on-chain program can be changed, updated, or deleted at any time causing this desktop app to stop functioning properly.
- This desktop app is also under active development. Use at your own discretion.


## Getting Started

### Download
To quickly get started, build are provided as releases. This project is currently in alpha.
Navigate to the Release, download the zip for your platform. Extract the zip into a folder at your desired location (A folder on Desktop will work).
This zip should have an assets folder, and an executable. Make sure the assets are always right next to your executable or it won't be able to find them and your screen will just be blank.

### Build from Source

To get started you will need to have rust installed. 

Drillx and the Ore Program are currently being updated, so you will need to clone those repos. Then make sure to update the paths for these dependencies in the Cargo.toml file for this app.
Once you have Drillx, Ore, and this app cloned, and have ensured the Cargo.toml paths are correct. Finally, run `cargo run --release` for the desktop app to build and run.
The build and compile may take a while.

### Running
Once the app is successfully running, you should see a config screen. You can provide your own rpc url, or leave it as the default.
For the best experience when interacting with the Solana network it is recommended that you get your own rpc and don't rely on the free public one for important work.

After the Config Setup screen you will need to generate/import a wallet. To generate a wallet, click generate, record your seed phrase, add a password or leave it empty for a blank password.
To import a .json wallet. Just drag-and-drop the .json file into the app and it will update the shown public key. Verify it's valid, add a password and click save.

Once the password is complete you will be taken to the Unlock screen. Here you must enter your previous password, and then click unlock or press `enter`.

Now you will be on the Mining Screen. Wait for the ui fetch interval to kick in and update all the balance. Then toggle auto-mine and auto-reset to start mining.

#### Notes:
Auto-Reset will occur ~5 seconds before your current challenge will need to be submitted. This ensure resets are only done as needed.

The Orange bus is the last bus that was used.
The Green flash of the bus is when the transaction was sent and processed on it.

Use the `c` key to get to the config screen again from the mining screen.
